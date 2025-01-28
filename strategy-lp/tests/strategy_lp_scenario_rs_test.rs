use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/strategy-lp.mxsc.json", strategy_lp::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/strategy_lp.scen.json");
}
