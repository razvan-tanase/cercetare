#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait StrategyLp {
    #[init]
    fn init(&self, asset_id: TokenIdentifier) {
        self.asset_id().set(asset_id);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ==========
    
    // ========== VIEWS ==========

    #[view(balanceOfAsset)]
    fn balance_of_asset(&self) -> BigUint {
        self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(self.asset_id().get()), 
            0
        )
    }

    #[view(balanceOf)]
    fn balance_of(&self) -> BigUint {
        self.balance_of_asset() // +
    }

    // ========== STORAGE ==========
    
    #[view(getAssetId)]
    #[storage_mapper("assetId")]
    fn asset_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
