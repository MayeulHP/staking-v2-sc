multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::structs::{BuildingAttributes, BuildingRarity, BuildingType, CitizenAttributes};

#[multiversx_sc::module]
pub trait UtilsModule: crate::common::CommonModule
{

    fn get_genesis_building_attributes(
        &self,
        nonce: &u64
    ) -> BuildingAttributes {
        let mut building_rarity = match nonce % 2 {
            1 => BuildingRarity::GenesisClassic,
            0 => BuildingRarity::GenesisLegendary,
            _ => unreachable!(),
        };
        let building_type = match nonce {
            1 | 2 => BuildingType::Headquarter,
            3 | 4 => BuildingType::Metro,
            5 | 6 => BuildingType::Park,
            7 | 8 => BuildingType::Police_Station,
            9 | 10 => BuildingType::Museum,
            11 | 12 => BuildingType::Hotel,
            13 | 14 => BuildingType::Casino,
            15 | 16 => BuildingType::TownHall,
            _ => BuildingType::None,
        };
        if building_type == BuildingType::TownHall {
            building_rarity = match nonce % 2 {
                1 => BuildingRarity::TownHallClassic,
                0 => BuildingRarity::TownHallLegendary,
                _ => unreachable!(),
            };
        }
        return BuildingAttributes {
            building_type: building_type,
            building_rarity: building_rarity
        };
    }

    fn get_expansion_building_attributes(
        &self,
        nonce: &u64
    ) -> BuildingAttributes {
        let building_rarity = match nonce % 3 {
            1 => BuildingRarity::ExpansionClassic,
            2 => BuildingRarity::ExpansionRare,
            0 => BuildingRarity::ExpansionLegendary,
            _ => unreachable!()
        };
        let building_type = match nonce {
            1 | 2 | 3 => BuildingType::Casino,
            4 | 5 | 6 => BuildingType::Restaurant,
            7 | 8 | 9 => BuildingType::Wine_Bar,
            10 | 11 | 12 => BuildingType::Airport,
            13 | 14 | 15 => BuildingType::Arcade_Room,
            16 | 17 | 18 => BuildingType::Stadium,
            19 | 20 | 21 => BuildingType::Theme_Park,
            22 | 23 | 24 => BuildingType::Rocket_Station,
            _ => unreachable!()
        };
        return BuildingAttributes {
            building_type: building_type,
            building_rarity: building_rarity
        }
    }

    fn get_building_attributes(
        &self,
        token_identifier: Option<TokenIdentifier>,
        nonce: &u64
    ) -> BuildingAttributes
    {
        if token_identifier.is_none() {
            return BuildingAttributes {
                building_type: BuildingType::None,
                building_rarity: BuildingRarity::None
            };
        }
        if token_identifier.unwrap() == self.genesis_tokenidentifier().get_token_id() {
            return self.get_genesis_building_attributes(nonce);
        } else {
            return self.get_expansion_building_attributes(nonce);
        }
        
    }

    fn get_citizen_attributes(
        &self,
        nonce: u64
    ) -> CitizenAttributes<Self::Api>
    {
        return self.blockchain().get_esdt_token_data(&self.blockchain().get_sc_address(), &self.citizen_tokenidentifier().get_token_id(), nonce).decode_attributes();

    }

    fn get_citizen_traits(
        &self,
        attributes: CitizenAttributes<Self::Api>
    ) -> ManagedVec<ManagedBuffer<Self::Api>>
    {
        let mut res: ManagedVec<ManagedBuffer<Self::Api>> = ManagedVec::new();
        res.push(attributes.face);
        res.push(attributes.hair);
        res.push(attributes.top);
        res.push(attributes.over_clothe);
        res.push(attributes.bottom);
        res.push(attributes.hat);
        res.push(attributes.held_item);
        res.push(attributes.eyes_object);
        res.push(attributes.mouth_object);
        res.push(attributes.extra);
        return res;
    }
}