#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait Strategy {
    
    // ========== INTERNALS ==========

    fn set_lp_pool_address(&self, lp_pool_address: ManagedAddress) {
        self.lp_pool_address().set(lp_pool_address);
    }

    // ========== VIEWS ==========

    #[view(getRewardTokenIdentifier)]
    fn get_lp_token_identifier(&self) -> TokenIdentifier {
        self.tx()
            .to(self.lp_pool_address().get())
            .typed(proxy::PairProxy)
            .get_lp_token_identifier()
            .returns(ReturnsResult)
            .sync_call_readonly()
    }

    // ========== STORAGE ==========
    
    #[view(getLpPoolAddress)]
    #[storage_mapper("lp_pool_address")]
    fn lp_pool_address(&self) -> SingleValueMapper<ManagedAddress>;
}