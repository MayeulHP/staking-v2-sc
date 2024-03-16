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
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ecity_tokenid (TokenIdentifier)
ARG_1=0x$(echo -n $2 | xxd -p -u | tr -d '\n')  # 1: gns_tokenid (TokenIdentifier)
ARG_2=0x$(echo -n $3 | xxd -p -u | tr -d '\n')  # 2: exp_tokenid (TokenIdentifier)
ARG_3=0x$(echo -n $4 | xxd -p -u | tr -d '\n')  # 3: ctzn_tokenid (TokenIdentifier)
ARG_4=${5}  # 4: router_address (Address)
    mxpy contract upgrade ${ADDRESS} --bytecode output/staking-v2-sc.wasm --recall-nonce ${PRIVATE_KEY} --keyfile "${OWNER}.json" --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --arguments ${ARG_0} ${ARG_1} ${ARG_2} ${ARG_3} ${ARG_4} 

}

# All contract endpoints are available as functions. Provide any arguments as needed (e.g transfer 12 TOKEN-123)

depositEcity() {
    token_name="0x$(echo -n ${TOKEN_IDENTIFIER} | xxd -p -u | tr -d '\n')"
    nonce=${TOKEN_NONCE}
    amount=$(echo "scale=0; (${1}*10^18)/1" | bc -l) # Lets you enter it as 0.05 instead of 50000000000000000
    sc_function="0x$(echo -n 'depositEcity' | xxd -p -u | tr -d '\n')"
    sc_address="0x$(mxpy wallet bech32 --decode ${ADDRESS})"

    mxpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY} \
            --gas-limit=50000000 \
            --proxy=${PROXY} --chain=${CHAIN_ID} \
            --function="ESDTTransfer" \
            --arguments ${token_name} ${amount} ${sc_function}\
            --send
    echo $?
}

depositEcity_deprecated() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "depositEcity" 
}

addEcity() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "addEcity" 
}

stake() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "stake" 
}

unstakeSingle() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: token_id (TokenIdentifier)
ARG_1=${2}  # 1: nonce (u64)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "unstakeSingle" \
        --arguments ${ARG_0} ${ARG_1} 

}

unstake() {
# Arguments: 
ARG_0=${1}  # 0: payments (variadic<tuple<TokenIdentifier,u64,u64>>)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "unstake" \
        --arguments ${ARG_0} 

}

claimEcity() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
ARG_1=${2}  # 1: addr (Address)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claimEcity" \
        --arguments ${ARG_0} ${ARG_1} 

}

claim() {
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claim" 
}

claimUnclaimable() {
# Arguments: 
ARG_0=${1}  # 0: episode (u64)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "claimUnclaimable" \
        --arguments ${ARG_0} 

}

setEcityTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ecity_tokenid (TokenIdentifier)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setEcityTokenid" \
        --arguments ${ARG_0} 

}

setGnsTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: gns_tokenid (TokenIdentifier)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setGnsTokenid" \
        --arguments ${ARG_0} 

}

setExpTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: exp_tokenid (TokenIdentifier)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setExpTokenid" \
        --arguments ${ARG_0} 

}

setCtznTokenid() {
# Arguments: 
ARG_0=0x$(echo -n $1 | xxd -p -u | tr -d '\n')  # 0: ctzn_tokenid (TokenIdentifier)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setCtznTokenid" \
        --arguments ${ARG_0} 

}

setRouterAddress() {
# Arguments: 
ARG_0=${1}  # 0: router_address (Address)
    mxpy contract call ${ADDRESS} \
        --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send \
        --function "setRouterAddress" \
        --arguments ${ARG_0} 

}

# All contract views. Provide arguments as needed (e.g balanceOf 0x1234567890123456789012345678901234567890)

nbStaked() {
# Arguments: 
ARG_0=${1}  # 0: user (Address)
    mxpy contract query ${ADDRESS} \
        --function "nbStaked" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

nbPlayers() {
    mxpy contract query ${ADDRESS} \
        --function "nbPlayers" \
        --proxy=${PROXY} 
}

fakeClaim() {
# Arguments: 
ARG_0=${1}  # 0: addr (Address)
    mxpy contract query ${ADDRESS} \
        --function "fakeClaim" \
        --proxy=${PROXY} \
         --arguments ${ARG_0} 

}

