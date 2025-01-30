// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    strategy_lp
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        getBalanceOfAsset => balance_of_asset
        getBalanceOf => balance_of
        getAssetId => asset_id
        getPoolAddress => pool_address
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
