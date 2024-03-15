from typing import List

'''

    EXAMPLE RUST FUNCTIONS

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
'''

class NFT:
    def __init__(self, token_id: str, nonce: int, quantity: int):
        self.token_id = token_id
        self.nonce = nonce
        self.quantity = quantity

# This script calculates the rewards for a given list of NFTs and the total rewards for a given episode.
# The rewards are calculated based on the rarity of the NFTs and the total rewards for the episode.

def calculate_rewards(episode: int, token_ids: List[NFT]) -> int:
    episode_rewards = 1000000
    total_rewards = 0

    for token in token_ids:
        if token.token_id == "genesis_classic":
            total_rewards += gns_reward(episode_rewards, "genesis_classic")
        elif token.token_id == "genesis_legendary":
            total_rewards += gns_reward(episode_rewards, "genesis_legendary")
        elif token.token_id == "expansion_classic":
            total_rewards += exp_reward(episode_rewards, "expansion_classic")
        elif token.token_id == "expansion_rare":
            total_rewards += exp_reward(episode_rewards, "expansion_rare")
        elif token.token_id == "expansion_legendary":
            total_rewards += exp_reward(episode_rewards, "expansion_legendary")
        elif token.token_id == "ctzn_classic":
            total_rewards += ctzn_reward(episode_rewards, "ctzn_classic")
        elif token.token_id == "ctzn_rare":
            total_rewards += ctzn_reward(episode_rewards, "ctzn_rare")
        elif token.token_id == "ctzn_legendary":
            total_rewards += ctzn_reward(episode_rewards, "ctzn_legendary")
        else:
            total_rewards += 0
    return total_rewards

def gns_reward(episode_rewards: int, building_rarity: str) -> int:
    building_fund = episode_rewards // 2
    gns_fund = building_fund // 2
    gns_rarity_fund = gns_fund // 2

    if building_rarity == "genesis_classic":
        return gns_rarity_fund // 961
    elif building_rarity == "genesis_legendary":
        return gns_rarity_fund // 31
    else:
        return 0
    
def exp_reward(episode_rewards: int, building_rarity: str) -> int:
    building_fund = episode_rewards // 2
    exp_fund = building_fund // 2
    exp_rarity_fund = exp_fund // 3
    
    if building_rarity == "expansion_classic":
        return exp_rarity_fund // 3560
    elif building_rarity == "expansion_rare":
        return exp_rarity_fund // 400
    elif building_rarity == "expansion_legendary":
        return exp_rarity_fund // 40
    else:
        return 0
    
def ctzn_reward(episode_rewards: int, ctzn_rarity: str) -> int:
    ctzn_fund = episode_rewards // 2
    ctzn_rarity_fund = ctzn_fund // 3
    
    if ctzn_rarity == "ctzn_classic":
        return ctzn_rarity_fund // 7120
    elif ctzn_rarity == "ctzn_rare":
        return ctzn_rarity_fund // 800
    elif ctzn_rarity == "ctzn_legendary":
        return ctzn_rarity_fund // 80
    else:
        return 0
    
def scale_rewards(total_rewards: int, time_staked: int) -> int:
    # Scale the rewards based on the time staked. The entire episode lasts 2 weeks. time_staked is the time staked in seconds.
    return total_rewards * (time_staked / 1209600) # 1209600 seconds = 2 weeks

# Example usage
nfts = [NFT("genesis_classic", 1, 1), NFT("genesis_legendary", 2, 1), NFT("expansion_classic", 3, 1), NFT("expansion_rare", 4, 1), NFT("expansion_legendary", 5, 1), NFT("ctzn_classic", 6, 1), NFT("ctzn_rare", 7, 1), NFT("ctzn_legendary", 8, 1)]
    