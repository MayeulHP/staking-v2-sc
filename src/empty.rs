#![no_std]

use multiversx_sc::storage;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Debug)]
pub struct MyEsdtTokenPayment<T: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<T>,
    pub token_nonce: u64,
    pub amount: BigUint<T>
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait StakingV2ScContract {
    #[init]
    fn init(&self) {}

    #[storage_mapper("ecityToken")]
    fn ecity_token(&self) -> FungibleTokenMapper<Self::Api>;

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

    #[payable("*")]
    #[endpoint(depositEcity)]
    fn deposit_ecity(&self) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.router_address().get(), "Only the router can deposit");
        let payment = self.call_value().single_esdt();
        require!(payment.token_identifier == self.ecity_token().get_token_id(), "Only ECITY can be deposited");

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
                self.staked(&caller, &payment.token_identifier, &payment.token_nonce).get() + payment.amount
            );
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
    fn claim_episode(&self, episode: u64) {
        let caller = self.blockchain().get_caller();

        require!(self.episodes_rewards(episode).get() > BigUint::from(0u8), "Nothing to claim");
        require!(self.current_episode().get() > episode, "Episode not yet available");

        let total_rewards = self.episodes_rewards(episode).get();

        let mut to_be_sent = BigUint::from(0u8);

        self.collections().iter().for_each(|token_id| {
            self.staked_iter(&caller, &token_id.clone()).iter().for_each(|(nonce, amount)| {
                if amount.eq(&BigUint::from(0u8)) {
                    return;
                }

                let reward = BigUint::from(10u32); //TODO Calculate reward

                to_be_sent += reward;

                self.staked_time(&caller, &token_id.clone(), &nonce).set(self.episodes_timestamps(episode).get()); // Move the staking time to the beginning of the episode
            });
        });

        self.send().direct_esdt(&caller, &self.ecity_token().get_token_id(), 0, &to_be_sent);
    }

    #[endpoint(claim)]
    fn claim(&self) {
        // Claims all unclaimed episodes
        let caller = self.blockchain().get_caller();

        let last_episode_claimed = self.last_episode_claimed(&caller).get();

        for episode in last_episode_claimed..self.current_episode().get() {
            self.claim_episode(episode);
        }

        self.last_episode_claimed(&caller).set(self.current_episode().get());
    }
}
