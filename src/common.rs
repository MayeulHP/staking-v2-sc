multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CommonModule {

     //-Tokens
    #[storage_mapper("genesis_tokenid")]
    fn genesis_tokenid(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("expansion_tokenid")]
    fn expansion_tokenid(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("citizen_tokenid")]
    fn citizen_tokenid(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("nbEvents")]
    fn nb_events(&self) -> SingleValueMapper<usize>;

    #[view(getVotes)]
    #[storage_mapper("votes")]
    fn votes(&self, nb_episode: u64) -> MapMapper<usize, BigUint>;

    #[storage_mapper("ecityTokenid")]
    fn ecity_tokenid(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("startTimestamp")]
    fn start_timestamp(&self) -> SingleValueMapper<u64>;

    fn get_day(
        &self
    ) -> u64
    {
        return (self.blockchain().get_block_timestamp() - self.start_timestamp().get()) / (60 * 60 * 24) % (14);
    }

    #[view(getEpisodeNumber)] //TEMPORARY VIEW FUNCTION
    fn get_episode_number(
        &self
    ) -> u64
    {
        return (self.blockchain().get_block_timestamp() - self.start_timestamp().get()) / (60 * 60 * 24 * 14);
    }

    #[view(getEpisodeTimeStamp)] //TEMPORARY VIEW FUNCTION
    fn get_episode_time_stamp(
        &self
    ) -> u64
    {
        return (60 * 60 * 24 * 14) * self.get_episode_number() + self.start_timestamp().get();
    }
    
    #[view(getVoteResult)] // Returns the vote result
    fn get_vote_result(
        &self
        ) -> usize
    {
        let day = self.get_day();
        if day == 0 {
            return 0;
        }
        
        // Gets the index of the maximum value of "votes"
        let nb_episode = self.get_episode_number();
        let max = self.votes(nb_episode).iter().max_by_key(|x| x.1.clone());
        return max.unwrap_or_default().0.into();
    }
}