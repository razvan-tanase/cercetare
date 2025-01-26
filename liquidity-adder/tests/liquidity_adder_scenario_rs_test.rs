use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/liquidity-adder.mxsc.json", liquidity_adder::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/liquidity_adder.scen.json");
}
