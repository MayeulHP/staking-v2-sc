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

    #[storage_mapper("ecityTokenid")]
    fn ecity_tokenid(&self) -> NonFungibleTokenMapper<Self::Api>;

}