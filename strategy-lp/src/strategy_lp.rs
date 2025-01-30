#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod proxy;

static WANTED_TOKEN_ID: &[u8] = b"WEGLD-a28c59";
static SWAP_TOKENS_FIXED_INPUT_FUNC_NAME: &[u8] = b"swapTokensFixedInput";
type SwapOperationType<M> =
    MultiValue4<ManagedAddress<M>, ManagedBuffer<M>, TokenIdentifier<M>, BigUint<M>>;
type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;

#[multiversx_sc::contract]
pub trait StrategyLp {
    #[init]
    fn init(&self, asset_id: TokenIdentifier, pool_address: ManagedAddress) {
        self.asset_id().set(asset_id);
        self.pool_address().set(pool_address);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ==========

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self, destination: ManagedAddress) -> PaymentsVec<Self::Api> {
        let (token_id, amount) = self.call_value().single_fungible_esdt();

        require!(token_id == self.asset_id().get(), "Invalid token id");

        let pair_address = self.pool_address().get();

        let mut swap_operations = MultiValueEncoded::new();
        swap_operations.push(SwapOperationType::from((
            pair_address.clone(),
            ManagedBuffer::from(SWAP_TOKENS_FIXED_INPUT_FUNC_NAME),
            TokenIdentifier::from(WANTED_TOKEN_ID),
            BigUint::from(1u64),
        )));

        let payment = EsdtTokenPayment::new(token_id, 0, amount);

        let a: PaymentsVec<Self::Api> = self.tx()
            .to(destination)
            .typed(proxy::AutoPosCreatorProxy)
            .create_lp_pos_from_single_token(
                pair_address, 
                BigUint::from(1u64), 
                BigUint::from(1u64), 
                swap_operations
            )
            .with_esdt_transfer(payment)
            .execute_on_dest_context();

        a
            
    }

    
    // ========== VIEWS ==========

    #[view(getBalanceOfAsset)]
    fn balance_of_asset(&self) -> BigUint {
        self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(self.asset_id().get()), 
            0
        )
    }

    #[view(getBalanceOf)]
    fn balance_of(&self) -> BigUint {
        self.balance_of_asset() // +
    }

    // ========== STORAGE ==========
    
    #[view(getAssetId)]
    #[storage_mapper("assetId")]
    fn asset_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPoolAddress)]
    #[storage_mapper("poolAddress")]
    fn pool_address(&self) -> SingleValueMapper<ManagedAddress>;
}
