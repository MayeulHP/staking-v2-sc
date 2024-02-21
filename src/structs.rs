multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct ConfigDTO<T: ManagedTypeApi> {
    pub genesis_tokenidentifier: TokenIdentifier<T>,
    pub expansion_tokenidentifier: TokenIdentifier<T>,
    pub citizen_tokenidentifier: TokenIdentifier<T>,
    pub max_building: u8,
    pub max_citizen: u8,
    pub game_status: bool,
    pub start_timestamp: u64,
    pub day: u64,
    pub episode_number: u64
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, PartialEq, Debug)]

pub struct Move<T: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<T>,
    pub nonce: u64,
    pub old_bat_pos: u8,
    pub old_cit_pos: u8,
    pub new_bat_pos: u8,
    pub new_cit_pos: u8
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Clone, PartialEq, Debug)]
pub struct Citizen<T: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<T>,
    pub nonce: u64,
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Clone, PartialEq, Debug)]
pub struct Building<T: ManagedTypeApi> {
    pub token_identifier: Option<TokenIdentifier<T>>,
    pub nonce: u64,
    pub citizens: ManagedVec<T, Option<Citizen<T>>>
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, PartialEq, Debug)]
pub struct Game<T: ManagedTypeApi> {
    pub buildings: ManagedVec<T, Building<T>>
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
pub struct Activator<T: ManagedTypeApi> {
    pub building_types: ManagedVec<T, BuildingType>,
    pub job_types: ManagedVec<T, ManagedBuffer<T>>,
    pub citizen_attributes: ManagedVec<T, ManagedBuffer<T>>,
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, Clone, PartialEq, ManagedVecItem, Debug)]
pub struct Effect<T: ManagedTypeApi> {
    pub boost_value: BigUint<T>,
    pub activation_day: BigUint<T>,
    pub activator: Option<Activator<T>>
}

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, Clone, PartialEq, ManagedVecItem, Debug)]
pub struct Event<T: ManagedTypeApi> {
    pub id: BigUint<T>,
    pub title: ManagedBuffer<T>,
    pub description: ManagedBuffer<T>,
    pub effects: ManagedVec<T, Effect<T>>
}

impl<T: ManagedTypeApi> Default for Event<T> {
    fn default() -> Self {
        Event {
            id: BigUint::zero(),
            title: ManagedBuffer::new(),
            description: ManagedBuffer::new(),
            effects: ManagedVec::new()
        }
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