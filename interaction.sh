# Replace the following with your own values (You need to run the script once to get the contract address)
ADDRESS="erd1qqq...xxx"
OWNER="erd1...xxx"
# Place your keystore file in the same directory as this script and replace the following with the name of the file
# Optionally, you can also put your password in the .passfile in the same directory as this script (if not, you will be prompted for the password)
PRIVATE_KEY=(--keyfile=erd1...xxx.json --passfile=.passfile)
PROXY=https://devnet-api.elrond.com
CHAIN_ID=D

# Standard deploy command. Provide any constructor arguments as needed (e.g deploy 12 TOKEN-123456). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
deploy() {
    erdpy contract build
    erdpy contract deploy --bytecode output/staking-v2-sc.wasm --recall-nonce ${PRIVATE_KEY} --keyfile ${OWNER}.json --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send --outfile="deploy.interaction.json" 
    echo "Deployed contract at the address written above."
    echo "Pleade update the ADDRESS variable in this script with the address of the deployed contract, then run 'source interaction.sh' to update the environment variables."
}

# Standard upgrade command. Provide any constructor arguments as needed (e.g upgrade 12 TOKEN-123). Numbers are automatically scaled to 18 decimals. (e.g. 12 -> 12000000000000000000)
upgrade() {
    erdpy contract upgrade ${ADDRESS} --bytecode output/staking-v2-sc.wasm --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send 
}

# All contract endpoints are available as functions. Provide any arguments as needed (e.g transfer 12 TOKEN-123)

depositEcity() {
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "depositEcity" 
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

fakeClaim() {
# Arguments: 
ARG_0=${1}  # 0: addr (Address)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "fakeClaim" \
        --arguments ${ARG_0} 

}

claimUnclaimable() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    erdpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claimUnclaimable" \
        --arguments ${ARG_0} 

}

# All contract views. Provide arguments as needed (e.g balanceOf 0x1234567890123456789012345678901234567890)

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

