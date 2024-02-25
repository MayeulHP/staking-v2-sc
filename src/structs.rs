multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Clone, PartialEq, Debug)]
pub struct Citizen<T: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<T>,
    pub nonce: u64,
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Debug)]
pub struct MyEsdtTokenPayment<T: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<T>,
    pub token_nonce: u64,
    pub amount: BigUint<T>
}

impl<T: ManagedTypeApi> PartialEq for MyEsdtTokenPayment<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.token_identifier == other.token_identifier && self.token_nonce == other.token_nonce && self.amount >= other.amount;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, Clone, PartialEq, ManagedVecItem, Debug)]
pub enum BuildingType {
    None,
    Headquarter,
    Metro,
    Park,
    PoliceStation,
    Museum,
    Hotel,
    Casino,
    Restaurant,
    Airport,
    ArcadeRoom,
    Stadium,
    ThemePark,
    RocketStation,
    WineBar,
    TownHall
}

impl Default for BuildingType {
    fn default() -> Self {
        BuildingType::None
    }
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, Clone, PartialEq, ManagedVecItem, Debug)]
pub enum BuildingRarity {
    None,
    GenesisClassic,
    GenesisLegendary,
    ExpansionClassic,
    ExpansionRare,
    ExpansionLegendary,
    TownHallClassic,
    TownHallLegendary
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, Clone, PartialEq, ManagedVecItem, Debug)]
pub struct BuildingAttributes {
    pub building_type: BuildingType,
    pub building_rarity: BuildingRarity
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode)]
pub struct CitizenAttributes<T: ManagedTypeApi> {
    pub id: u64,
    pub rank: u64,
    pub rarity_level: u64,
    pub job: ManagedBuffer<T>,
    pub building: ManagedBuffer<T>,
    pub face: ManagedBuffer<T>,
    pub hair: ManagedBuffer<T>,
    pub top: ManagedBuffer<T>,
    pub over_clothe: ManagedBuffer<T>,
    pub bottom: ManagedBuffer<T>,
    pub hat: ManagedBuffer<T>,
    pub held_item: ManagedBuffer<T>,
    pub eyes_object: ManagedBuffer<T>,
    pub mouth_object: ManagedBuffer<T>,
    pub extra: ManagedBuffer<T>,
}