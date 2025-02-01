use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/strategy-stake.mxsc.json", strategy_stake::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/strategy_stake.scen.json");
}
