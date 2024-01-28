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

    /* TODO: Add an iteratable staked tokens
    // Using a Vec per user to be able to iterate over all staked tokens
    #[storage_mapper("staked")]
    fn staked(&self, user: &ManagedAddress, token_id: &TokenIdentifier) -> VecMapper<(u64, BigUint<Self::Api>)>;
    */

    // TODO: Check how to handle SFTs (Multiple SFTs have the same nonce and token_id)
    #[storage_mapper("stakedTime")]
    fn staked_time(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[storage_mapper("unstaked_time")]
    fn unstaked_time(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nonce: &u64) -> SingleValueMapper<u64>;

    #[storage_mapper("collections")]
    fn collections(&self) -> SetMapper<TokenIdentifier<Self::Api>>;

    // Dates the beginning of each episode, starting from 1
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

        self.current_episode().set(self.current_episode().get() + 1);
        self.episodes_rewards(self.current_episode().get()).set(payment.amount);
        self.episodes_timestamps(self.current_episode().get()).set(self.blockchain().get_block_timestamp());
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

    #[endpoint(claim)]
    fn claim(&self, episode: u64) {
        let caller = self.blockchain().get_caller();

        require!(self.episodes_rewards(episode).get() > BigUint::from(0u8), "Nothing to claim");
        require!(self.current_episode().get() > episode, "Episode not yet available");

        let total_rewards = self.episodes_rewards(episode).get();

        let user_rewards = total_rewards; //TODO: Not implemented yet

        //TODO Continue here
    }

    // TODO: Move to constructor
    #[only_owner]
    #[endpoint(startFirstEpisode)]
    fn start_first_episode(&self) {
        self.current_episode().set(1);
        self.episodes_timestamps(1).set(self.blockchain().get_block_timestamp());
    }
}
