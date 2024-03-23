# Replace the following with your own values (You need to run the script once to get the contract address)
ADDRESS="erd1qqqqqqqqqqqqqpgq75u6xaaa4yfzrddhpvxzwqrdvt7wjzxfsdpstk9fzx"
OWNER="erd1hpcwz6fl7jeytmq0thkc0xu783sjlh9c3xx29rtsrd7ks09xsdps6m3cyu"
# Place your keystore file in the same directory as this script and replace the following with the name of the file
# Optionally, you can also put your password in the .passfile in the same directory as this script (if not, you will be prompted for the password)
PRIVATE_KEY=(--keyfile=erd1hpcwz6fl7jeytmq0thkc0xu783sjlh9c3xx29rtsrd7ks09xsdps6m3cyu.json --passfile=.passfile)
PROXY=https://gateway.multiversx.com
CHAIN_ID=1
TOKEN_IDENTIFIER="ECITY-2a383a"
TOKEN_NONCE=0

# Standard deploy command. Provide any constructor arguments as needed (e.g deploy 12 TOKEN-123456). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
deploy() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ecity_tokenid (TokenIdentifier)
ARG_1=0x$(echo -n $2 | xxd -p -u | tr -d '\n')  # 1: gns_tokenid (TokenIdentifier)
ARG_2=0x$(echo -n $3 | xxd -p -u | tr -d '\n')  # 2: exp_tokenid (TokenIdentifier)
ARG_3=0x$(echo -n $4 | xxd -p -u | tr -d '\n')  # 3: ctzn_tokenid (TokenIdentifier)
ARG_4=${5}  # 4: router_address (Address)
    mxpy contract build
    mxpy contract deploy --bytecode output/staking-v2-sc.wasm --recall-nonce ${PRIVATE_KEY} --keyfile ${OWNER}.json --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send --outfile="deploy.interaction.json" \
        --arguments ${ARG_0} ${ARG_1} ${ARG_2} ${ARG_3} ${ARG_4} 

    echo "Deployed contract at the address written above."
    echo "Pleade update the ADDRESS variable in this script with the address of the deployed contract, then run 'source interaction.sh' to update the environment variables."
}

# Standard upgrade command. Provide any constructor arguments as needed (e.g upgrade 12 TOKEN-123). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
upgrade() {
# Arguments: 
#ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ecity_tokenid (TokenIdentifier)
#ARG_1=0x$(echo -n $2 | xxd -p -u | tr -d '\n')  # 1: gns_tokenid (TokenIdentifier)
#ARG_2=0x$(echo -n $3 | xxd -p -u | tr -d '\n')  # 2: exp_tokenid (TokenIdentifier)
#ARG_3=0x$(echo -n $4 | xxd -p -u | tr -d '\n')  # 3: ctzn_tokenid (TokenIdentifier)
#ARG_4=${5}  # 4: router_address (Address)
    mxpy contract upgrade ${ADDRESS} --bytecode output/staking-v2-sc.wasm --recall-nonce ${PRIVATE_KEY} --keyfile "${OWNER}.json" --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send

}

# All contract endpoints are available as functions. Provide any arguments as needed (e.g transfer 12 TOKEN-123)

upgrade() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "upgrade" 
}

depositEcity() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "depositEcity" 
}

addEcity() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "addEcity" 
}

stake() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "stake" 
}

unstakeSingle() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: token_id (TokenIdentifier)
ARG_1=${2}  # 1: nonce (u64)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "unstakeSingle" \
        --arguments ${ARG_0} ${ARG_1} 

}

unstake() {
# Arguments: 
ARG_0=${1}  # 0: payments (variadic<tuple<TokenIdentifier,u64,u64>>)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "unstake" \
        --arguments ${ARG_0} 

}

claimEcity() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
ARG_1=${2}  # 1: addr (Address)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claimEcity" \
        --arguments ${ARG_0} ${ARG_1} 

}

claim() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claim" 
}

claimUnclaimable() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claimUnclaimable" \
        --arguments ${ARG_0} 

}

setEcityTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ecity_tokenid (TokenIdentifier)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setEcityTokenid" \
        --arguments ${ARG_0} 

}

setGnsTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: gns_tokenid (TokenIdentifier)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setGnsTokenid" \
        --arguments ${ARG_0} 

}

setExpTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: exp_tokenid (TokenIdentifier)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setExpTokenid" \
        --arguments ${ARG_0} 

}

setCtznTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ctzn_tokenid (TokenIdentifier)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setCtznTokenid" \
        --arguments ${ARG_0} 

}

setRouterAddress() {
# Arguments: 
ARG_0=${1}  # 0: router_address (Address)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setRouterAddress" \
        --arguments ${ARG_0} 

}

addToCollections() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: token_id (TokenIdentifier)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "addToCollections" \
        --arguments ${ARG_0} 

}

# All contract views. Provide arguments as needed (e.g balanceOf 0x1234567890123456789012345678901234567890)

routerAddress() {
    erdpy contract query ${ADDRESS} \
        --function "routerAddress" \
        --proxy=${PROXY} 
}

staked() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
ARG_1=0x$(echo -n $2 | xxd -p -u | tr -d '\n')  # 1: token_id (TokenIdentifier)
ARG_2=${3}  # 2: nonce (u64)
    erdpy contract query ${ADDRESS} \
        --function "staked" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} ${ARG_1} ${ARG_2} 

}

stakedIter() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
ARG_1=0x$(echo -n $2 | xxd -p -u | tr -d '\n')  # 1: token_id (TokenIdentifier)
    erdpy contract query ${ADDRESS} \
        --function "stakedIter" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} ${ARG_1} 

}

stakedTime() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
    erdpy contract query ${ADDRESS} \
        --function "stakedTime" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

nbStaked() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
    erdpy contract query ${ADDRESS} \
        --function "nbStaked" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

nbPlayers() {
    erdpy contract query ${ADDRESS} \
        --function "nbPlayers" \
        --proxy=${PROXY} 
}

lastEpisodeClaimed() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
    erdpy contract query ${ADDRESS} \
        --function "lastEpisodeClaimed" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

episodesTimestamps() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    erdpy contract query ${ADDRESS} \
        --function "episodesTimestamps" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

episodesRewards() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    erdpy contract query ${ADDRESS} \
        --function "episodesRewards" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

currentEpisode() {
    erdpy contract query ${ADDRESS} \
        --function "currentEpisode" \
        --proxy=${PROXY} 
}

claimedPerEpisode() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    erdpy contract query ${ADDRESS} \
        --function "claimedPerEpisode" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

fakeClaim() {
# Arguments: 
ARG_0=${1}  # 0: addr (Address)
    erdpy contract query ${ADDRESS} \
        --function "fakeClaim" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

