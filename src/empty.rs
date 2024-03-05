#![no_std]

//use multiversx_sc::storage;

multiversx_sc::imports!();

pub mod structs;
pub mod utils;
pub mod common;

use crate::structs::{MyEsdtTokenPayment, BuildingRarity, BuildingType};

/// The staking contract for ctzn.city
#[multiversx_sc::contract]
pub trait StakingV2ScContract:
    common::CommonModule
    + utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    #[storage_mapper("routerAddress")]
    fn router_address(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>;

    #[storage_mapper("staked")]
    fn staked(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<BigUint<Self::Api>>;

    // TODO: Add an iteratable staked tokens
    // Using a Vec per user to be able to iterate over all staked tokens
    #[storage_mapper("stakedIter")]
    fn staked_iter(&self, user: &ManagedAddress, token_id: &TokenIdentifier) -> VecMapper<(u64, BigUint<Self::Api>)>; // Contains (nonce, amount)

    #[storage_mapper("stakedTime")]
    fn staked_time(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(nbStaked)]
    #[storage_mapper("nbStaked")]
    fn nb_staked(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(nbPlayers)]
    #[storage_mapper("nbPlayers")]
    fn nb_players(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("lastEpisodeClaimed")]
    fn last_episode_claimed(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("collections")]
    fn collections(&self) -> SetMapper<TokenIdentifier<Self::Api>>;

    // Dates the beginning of each episode, starting from 0
    #[storage_mapper("episodesTimestamps")]
    fn episodes_timestamps(&self, episode: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("episodesRewards")]
    fn episodes_rewards(&self, episode: u64) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("currentEpisode")]
    fn current_episode(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("claimedPerEpisode")]
    fn claimed_per_episode(&self, episode: u64) -> SingleValueMapper<BigUint<Self::Api>>; // Will be used to allow the Team to claim unclaimable rewards

    // Simple function to cap time differences to 2 weeks
    fn cap_time(&self, time: u64) -> u64 {
        let two_weeks = 14 * 24 * 60 * 60; // The length of an episode is 2 weeks, in seconds

        if time > two_weeks {
            return two_weeks;
        }
        return time;
    }

    fn gns_reward(&self, episode_rewards: BigUint, building_rarity: BuildingRarity) -> BigUint<Self::Api> {

        let building_fund = episode_rewards / BigUint::from(2u8); // 50% of the rewards go to the building fund, the other 50% go to the citizens
        let gns_fund = building_fund / BigUint::from(2u8); // 50% of the rewards go to the genesis fund, the other 50% go to the expansion fund
        let gns_rarity_fund = gns_fund.clone() / BigUint::from(2u8); // 50% of the rewards go to the Genesis Classic fund, the other 50% go to the Genesis Legendary fund

        let building_reward = match building_rarity {
            BuildingRarity::GenesisClassic => gns_rarity_fund / BigUint::from(961u16), // 961 Genesis Classic buildings
            BuildingRarity::GenesisLegendary => gns_rarity_fund / BigUint::from(31u8), // 31 Genesis Legendary buildings
            BuildingRarity::TownHallClassic => gns_rarity_fund / BigUint::from(961u16), // 961 Genesis Classic buildings
            BuildingRarity::TownHallLegendary => gns_rarity_fund / BigUint::from(31u8), // 31 Genesis Legendary buildings,
            _ => BigUint::from(0u8)
        };

        return  building_reward;
    }

    fn exp_reward(&self, episode_rewards: BigUint, building_rarity: BuildingRarity) -> BigUint<Self::Api> {
        let building_fund = episode_rewards / BigUint::from(2u8); // 50% of the rewards go to the building fund, the other 50% go to the citizens
        let exp_fund = building_fund / BigUint::from(2u8); // 50% of the rewards go to the expansion fund, the other 50% go to the genesis fund
        let exp_rarity_fund = exp_fund.clone() / BigUint::from(3u8); // There are 3 rarities of Expansion buildings

        let building_reward = match building_rarity {
            BuildingRarity::ExpansionClassic => exp_rarity_fund / BigUint::from(3500u16), // 3500 Expansion Classic buildings
            BuildingRarity::ExpansionRare => exp_rarity_fund / BigUint::from(400u16), // 400 Expansion Rare buildings
            BuildingRarity::ExpansionLegendary => exp_rarity_fund / BigUint::from(40u8), // 40 Expansion Legendary buildings
            _ => BigUint::from(0u8)
        };

        return building_reward;
    }

    fn ctzn_reward(&self, episode: u64, nonce: u64) -> BigUint<Self::Api> {
        let episode_rewards = self.episodes_rewards(episode).get();
        let ctzn_attributes = self.get_citizen_attributes(nonce);

        let ctzn_fund = episode_rewards / BigUint::from(2u8); // 50% of the rewards go to the citizens, the other 50% go to the buildings
        let ctzn_rarity_fund = ctzn_fund / BigUint::from(3u8); // There are 3 rarities of citizens

        let citizen_reward = match ctzn_attributes.rarity_level {
            1 => ctzn_rarity_fund / BigUint::from(7120u16), // 1000 Common citizens
            2 => ctzn_rarity_fund / BigUint::from(800u16), // 100 Rare citizens
            3 => ctzn_rarity_fund / BigUint::from(80u8), // 10 Legendary citizens
            _ => BigUint::from(0u8)
        };

        return citizen_reward;
    }

    fn building_reward(&self, episode: u64, token_id: &TokenIdentifier, nonce: u64) -> BigUint<Self::Api> {
        let episode_rewards = self.episodes_rewards(episode).get();

        let attributes = self.get_building_attributes(Option::Some(token_id.clone()), &nonce);

        // Separate the case between the collections.
        // If building_rarity is GenesisClassic, GenesisLegendary, TownhallClassic or TownhallLegendary, it's a Genesis building.
        // If building_rarity is ExpansionClassic, ExpansionRare or ExpansionLegendary, it's an Expansion building.
        
        if attributes.building_rarity == BuildingRarity::GenesisClassic ||
            attributes.building_rarity == BuildingRarity::GenesisLegendary ||
            attributes.building_rarity == BuildingRarity::TownHallClassic ||
            attributes.building_rarity == BuildingRarity::TownHallLegendary
        {
            return self.gns_reward(episode_rewards, attributes.building_rarity);
        } else if attributes.building_rarity == BuildingRarity::ExpansionClassic ||
                    attributes.building_rarity == BuildingRarity::ExpansionLegendary {
            return self.gns_reward(episode_rewards, attributes.building_rarity);
        }

        return BigUint::from(0u8);
    }

    fn reward(&self, episode: u64, token_id: &TokenIdentifier, nonce: u64) -> BigUint<Self::Api> {
        let attributes = self.get_building_attributes(Option::Some(token_id.clone()), &nonce);

        if attributes.building_type != BuildingType::None {
            return self.building_reward(episode, token_id, nonce);
        } else {
            return self.ctzn_reward(episode, nonce);
        }
    }

    // Depositing ECITY starts and episode, indexed starting from 0
    #[payable("*")]
    #[endpoint(depositEcity)]
    fn deposit_ecity(&self) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.router_address().get(), "Only the router can deposit");
        let payment = self.call_value().single_esdt();
        require!(payment.token_identifier == self.ecity_tokenid().get_token_id(), "Only ECITY can be deposited");

        let current_episode = self.current_episode().get();

        if current_episode == 0 && self.episodes_timestamps(0u64).get() == 0u64 {
            // Do nothing if the current episode is zero and hasn't started yet
        } else {
            self.current_episode().set(current_episode + 1);
        }

        self.episodes_rewards(self.current_episode().get()).set(payment.amount);
        self.episodes_timestamps(self.current_episode().get()).set(self.blockchain().get_block_timestamp());
    }

    // Adding more ECITY to the episode without starting a new one
    #[payable("*")]
    #[endpoint(addEcity)]
    fn add_ecity(&self) {
        let payment = self.call_value().single_esdt();
        require!(payment.token_identifier == self.ecity_tokenid().get_token_id(), "Only ECITY can be deposited");

        let current_episode = self.current_episode().get();

        self.episodes_rewards(current_episode).set(self.episodes_rewards(current_episode).get() + payment.amount);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<MyEsdtTokenPayment<Self::Api>> = self.call_value().all_esdt_transfers().iter().map(
            |p| MyEsdtTokenPayment {
                token_identifier: p.token_identifier,
                token_nonce: p.token_nonce,
                amount: p.amount
            }).collect();
        
        let caller = self.blockchain().get_caller();

        // Claim all episodes since stake_time, and update the stake_time
        self.claim();

        for payment in payments.iter() {
            require!(self.collections().contains(&payment.token_identifier), "Token not supported");

            self.staked(&caller, &payment.token_identifier, &payment.token_nonce).set(
                self.staked(&caller, &payment.token_identifier, &payment.token_nonce).get() + &payment.amount
            );

            // Add the amount to the staked_iter, if it's not already there
            if !self.staked_iter(&caller, &payment.token_identifier).iter().any(|(nonce, _)| nonce == payment.token_nonce) {
                self.staked_iter(&caller, &payment.token_identifier).push(&(payment.token_nonce, payment.amount.clone()));
            } else {
                // If it's already there, update the amount
                self.staked_iter(&caller, &payment.token_identifier).iter().for_each(|(nonce, mut amount)| {
                    if nonce == payment.token_nonce {
                        amount += &payment.amount;
                    }
                });
            }

            // Update the number of players if nessessary
            if self.nb_staked(&caller).get() == 0 {
                self.nb_players().set(self.nb_players().get() + 1);
            }

            let payment_amount_u64 = payment.amount.to_u64().unwrap();

            self.nb_staked(&caller).set(self.nb_staked(&caller).get() + u64::from(payment_amount_u64));
        }
    }

    #[endpoint(unstakeSingle)]
    fn unstake_single(&self, token_id: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();

        require!(self.staked(&caller, &token_id, &nonce).get() > BigUint::from(0u8), "Nothing to unstake");

        // Before unstaking, claim all episodes since stake_time
        self.claim();

        self.staked(&caller, &token_id, &nonce).set(self.staked(&caller, &token_id, &nonce).get() - BigUint::from(1u8));

        if self.staked(&caller, &token_id, &nonce).get() == BigUint::from(0u8) {
            let index = self.staked_iter(&caller, &token_id).iter().position(|(n, _)| n == nonce).unwrap();
            self.staked_iter(&caller, &token_id).swap_remove(index);
        } else {
            self.staked_iter(&caller, &token_id).iter().for_each(|(n, mut amount)| {
                if n == nonce {
                    if amount > BigUint::from(1u8) {
                        amount -= BigUint::from(1u8);
                    } else {
                        let index = self.staked_iter(&caller, &token_id).iter().position(|(n, _)| n == nonce).unwrap();
                        self.staked_iter(&caller, &token_id).swap_remove(index);
                    }
                }
            });
        }

        self.nb_staked(&caller).set(self.nb_staked(&caller).get() - 1);

        if self.nb_staked(&caller).get() == 0 {
            self.nb_players().set(self.nb_players().get() - 1);
        }
        
        self.send().direct_esdt(&caller, &token_id, nonce, &BigUint::from(1u8));
    }

    // Unstakes a list of tokens
    #[endpoint(unstake)]
    fn unstake(&self, payments: MultiValueEncoded<(TokenIdentifier, u64, u64)>) {

        for (tokenid, nonce, quantity) in payments.into_iter() {
            for _ in 0..quantity {
            self.unstake_single(tokenid.clone(), nonce.clone());
            }
        }
    }

    #[endpoint(claimEcity)]
    fn claim_ecity(&self, episode: u64, addr: &ManagedAddress) {
        //require!(self.episodes_rewards(episode).get() > BigUint::from(0u8), "Nothing to claim");
        //require!(self.current_episode().get() > episode, "Episode not yet available");

        if self.episodes_rewards(episode).get() <= BigUint::zero() {
            return;
        }

        let now = self.blockchain().get_block_timestamp();
        let stake_time = self.staked_time(&addr).get();
        
        // Check that the stake time is in the episode being claimed
        if stake_time > self.episodes_timestamps(episode).get() + 1209600u64 {
            return;
        }

        let staked_length = self.cap_time(now - stake_time);

        let mut to_be_sent = BigUint::from(0u8);

        self.collections().iter().for_each(|token_id| {
            self.staked_iter(&addr, &token_id.clone()).iter().for_each(|(nonce, amount)| {
                if amount.eq(&BigUint::from(0u8)) {
                    return;
                }

                let reward = self.reward(episode, &token_id.clone(), nonce);

                to_be_sent += reward;
            });
        });

        to_be_sent = to_be_sent * staked_length / BigUint::from(1209600u64); // 1209600 seconds = 2 weeks

        self.claimed_per_episode(episode).set(self.claimed_per_episode(episode).get() + &to_be_sent);

        // If the episode being claimed is the current episode, set the stake time to now, else set it to the end of the episode being claimed
        if episode == self.current_episode().get() {
            self.staked_time(&addr).set(now);
        } else {
            self.staked_time(&addr).set(self.episodes_timestamps(episode).get() + 1209600u64);
        }

        self.send().direct_esdt(&addr, &self.ecity_tokenid().get_token_id(), 0, &to_be_sent);
    }

    #[endpoint(claim)]
    fn claim(&self) {
        // Claims all unclaimed episodes
        let caller = self.blockchain().get_caller();

        let last_episode_claimed = self.last_episode_claimed(&caller).get();

        for episode in last_episode_claimed..self.current_episode().get() {
            self.claim_ecity(episode, &caller);
        }

        self.last_episode_claimed(&caller).set(self.current_episode().get());
    }

    //#[view(fakeClaim)]
    #[endpoint(fakeClaim)]
    fn fake_claim(&self, addr: &ManagedAddress) -> BigUint<Self::Api> {
        // Counts the amount of ECITY that can be claimed by an address if the claim function was called

        let last_episode_claimed = self.last_episode_claimed(&addr).get();

        let mut to_be_sent = BigUint::from(0u8);

        for episode in last_episode_claimed..self.current_episode().get() {
            if self.episodes_rewards(episode).get() <= BigUint::zero() {
                continue;
            }

            let now = self.blockchain().get_block_timestamp();
            let stake_time = self.staked_time(&addr).get();
            
            // Check that the stake time is in the episode being claimed
            if stake_time > self.episodes_timestamps(episode).get() + 1209600u64 {
                continue;
            }

            let staked_length = self.cap_time(now - stake_time);

            self.collections().iter().for_each(|token_id| {
                self.staked_iter(&addr, &token_id.clone()).iter().for_each(|(nonce, amount)| {
                    if amount.eq(&BigUint::from(0u8)) {
                        return;
                    }

                    let reward = self.reward(episode, &token_id.clone(), nonce);

                    to_be_sent += reward;
                });
            });

            to_be_sent = to_be_sent * staked_length / BigUint::from(1209600u64); // 1209600 seconds = 2 weeks
        }

        return to_be_sent;
    }

    #[only_owner]
    #[endpoint(claimUnclaimable)]
    fn claim_unclaimable(&self, episode: u64) {
        // Allows the Team to claim leftover rewards from an episode
        let caller = self.blockchain().get_caller();

        require!(self.episodes_rewards(episode).get() > BigUint::from(0u8), "Nothing to claim");
        require!(self.current_episode().get() > episode, "Episode not yet available");

        let total_rewards = self.episodes_rewards(episode).get();

        let to_be_sent = total_rewards - self.claimed_per_episode(episode).get();

        self.send().direct_esdt(&caller, &self.ecity_tokenid().get_token_id(), 0, &to_be_sent);
    }
}
