use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/staking-v2-sc.wasm", staking_v2_sc::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/setup.scen.json");
}
