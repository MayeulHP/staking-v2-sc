#![no_std]

//use multiversx_sc::storage;

multiversx_sc::imports!();

pub mod structs;
pub mod utils;
pub mod common;

use crate::structs::{MyEsdtTokenPayment, Building, Citizen, BuildingRarity, BuildingType, CitizenAttributes};

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait StakingV2ScContract:
    common::CommonModule
    + utils::UtilsModule
{
    #[init]
    fn init(&self) {}

    //#[storage_mapper("ecityToken")]
    //fn ecity_token(&self) -> FungibleTokenMapper<Self::Api>;

    #[storage_mapper("routerAddress")]
    fn router_address(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>;

    #[storage_mapper("staked")]
    fn staked(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<BigUint<Self::Api>>;

    // TODO: Add an iteratable staked tokens
    // Using a Vec per user to be able to iterate over all staked tokens
    #[storage_mapper("stakedIter")]
    fn staked_iter(&self, user: &ManagedAddress, token_id: &TokenIdentifier) -> VecMapper<(u64, BigUint<Self::Api>)>; // Contains (nonce, amount)
    

    // TODO: Check how to handle SFTs (Multiple SFTs have the same nonce and token_id)
    #[storage_mapper("stakedTime")]
    fn staked_time(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[storage_mapper("unstaked_time")] // Might not be needed
    fn unstaked_time(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

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

    fn ctzn_reward(&self, episode_rewards: BigUint, ctzn_attribues: CitizenAttributes<Self::Api>) -> BigUint<Self::Api> {
        let ctzn_fund = episode_rewards / BigUint::from(2u8); // 50% of the rewards go to the citizens, the other 50% go to the buildings
        let ctzn_rarity_fund = ctzn_fund / BigUint::from(3u8); // There are 3 rarities of citizens

        let citizen_reward = match ctzn_attribues.rarity_level {
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
            return episode_rewards * BigUint::from(1u8) / BigUint::from(20u8);
        }

        return BigUint::from(0u8);
    }

    fn reward(&self, episode: u64, token_id: &TokenIdentifier, nonce: u64) -> BigUint<Self::Api> {
        let attributes = self.get_building_attributes(Option::Some(token_id.clone()), &nonce);

        if attributes.building_type != BuildingType::None {
            return self.building_reward(episode, token_id, nonce);
        }

        return BigUint::from(0u8);
    }

    #[payable("*")]
    #[endpoint(depositEcity)]
    fn deposit_ecity(&self) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.router_address().get(), "Only the router can deposit");
        let payment = self.call_value().single_esdt();
        require!(payment.token_identifier == self.ecity_tokenid().get_token_id(), "Only ECITY can be deposited");

        let current_episode = self.current_episode().get();

        if current_episode == 0 && self.episodes_rewards(current_episode).get() == BigUint::from(0u8) {
            // Do nothing if the current episode is zero and has no rewards, since the first episode hasn't started yet
        } else {
            self.current_episode().set(current_episode + 1);
        }

        self.episodes_rewards(current_episode + 1).set(payment.amount);
        self.episodes_timestamps(current_episode+ 1).set(self.blockchain().get_block_timestamp());
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

        for payment in payments.iter() {
            require!(self.collections().contains(&payment.token_identifier), "Token not supported");

            self.staked(&caller, &payment.token_identifier, &payment.token_nonce).set(
                self.staked(&caller, &payment.token_identifier, &payment.token_nonce).get() + &payment.amount
            );

            // Add the amount to the staked_iter, if it's not already there
            if !self.staked_iter(&caller, &payment.token_identifier).iter().any(|(nonce, _)| nonce == payment.token_nonce) {
                self.staked_iter(&caller, &payment.token_identifier).push(&(payment.token_nonce, payment.amount));
            } else {
                // If it's already there, update the amount
                self.staked_iter(&caller, &payment.token_identifier).iter().for_each(|(nonce, mut amount)| {
                    if nonce == payment.token_nonce {
                        amount += &payment.amount;
                    }
                });
            }

            self.staked_time(&caller, &payment.token_identifier, &payment.token_nonce).set(self.blockchain().get_block_timestamp());
        }
    }

    #[endpoint(unstakeSingle)]
    fn unstake_single(&self, token_id: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();

        require!(self.staked(&caller, &token_id, &nonce).get() > BigUint::from(0u8), "Nothing to unstake");

        self.staked(&caller, &token_id, &nonce).set(self.staked(&caller, &token_id, &nonce).get() - BigUint::from(1u8));
        self.unstaked_time(&caller, &token_id, &nonce).set(self.blockchain().get_block_timestamp()); //TODO Check if more logic is needed for SFTs. (Might need to only update the timestamp if it's the first unstake)
        self.send().direct_esdt(&caller, &token_id, nonce, &BigUint::from(1u8));
    }

    #[endpoint(claimEpisode)]
    fn claim_episode(&self, episode: u64, addr: &ManagedAddress) {
        require!(self.episodes_rewards(episode).get() > BigUint::from(0u8), "Nothing to claim");
        require!(self.current_episode().get() > episode, "Episode not yet available");

        let total_rewards = self.episodes_rewards(episode).get();

        let mut to_be_sent = BigUint::from(0u8);

        self.collections().iter().for_each(|token_id| {
            self.staked_iter(&addr, &token_id.clone()).iter().for_each(|(nonce, amount)| {
                if amount.eq(&BigUint::from(0u8)) {
                    return;
                }

                let reward = self.reward(episode, &token_id.clone(), nonce);


                to_be_sent += reward;

                self.staked_time(&addr, &token_id.clone(), &nonce).set(self.episodes_timestamps(episode).get()); // Move the staking time to the beginning of the episode
            });
        });

        self.claimed_per_episode(episode).set(self.claimed_per_episode(episode).get() + &to_be_sent);

        self.send().direct_esdt(&addr, &self.ecity_tokenid().get_token_id(), 0, &to_be_sent);
    }

    #[endpoint(claim)]
    fn claim(&self) {
        // Claims all unclaimed episodes
        let caller = self.blockchain().get_caller();

        let last_episode_claimed = self.last_episode_claimed(&caller).get();

        for episode in last_episode_claimed..self.current_episode().get() {
            self.claim_episode(episode, &caller);
        }

        self.last_episode_claimed(&caller).set(self.current_episode().get());
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
