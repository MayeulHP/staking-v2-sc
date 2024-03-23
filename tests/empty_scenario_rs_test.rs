use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/staking-v2-sc.wasm", staking_v2_sc::ContractBuilder);
    blockchain
}

#[test]
fn long_stake_rs() {
    world().run("scenarios/setup.scen.json");
}

#[test]
fn double_deposit() {
    world().run("scenarios/double_deposit.scen.json");
}

#[test]
fn stake_one() {
    world().run("scenarios/stake_one.scen.json");
}

#[test]
fn complex_stake() {
    world().run("scenarios/complex_stake.scen.json");
}


#[test]
fn long_stake() {
    world().run("scenarios/long_stake.scen.json");
}

#[test]
fn stake_twice() {
    world().run("scenarios/stake_twice.scen.json");
}