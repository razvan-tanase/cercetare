#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod proxy;

#[multiversx_sc::contract]
pub trait StrategyStake {
    #[init]
    fn init(&self, asset_id: TokenIdentifier, farm_address: ManagedAddress) {
        require!(asset_id.is_valid_esdt_identifier(), "Invalid token ID");
        require!(
            !farm_address.is_zero() && self.blockchain().is_smart_contract(&farm_address),
            "Invalid SC address"
        );

        self.asset_id().set(asset_id);
        self.farm_address().set(farm_address);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ==========

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        let (token_id, amount) = self.call_value().single_fungible_esdt();
        require!(token_id == self.asset_id().get(), "Invalid token ID");
        
        let farm_address = self.farm_address().get();
        let payment = EsdtTokenPayment::new(token_id, 0, amount);
            
        let stake_result = self.tx()
            .to(farm_address)
            .typed(proxy::FarmStakingProxy)
            .stake_farm_endpoint(OptionalValue::None::<ManagedAddress>)
            .payment(payment)
            .returns(ReturnsResult)
            .sync_call()
            .into_tuple();

        self.last_stake_result().set(stake_result);
    }

    #[payable("*")]
    #[endpoint(withdraw)]
    fn withdraw(&self, amount: BigUint) {
        let farm_address = self.farm_address().get();
        let contract_address = self.blockchain().get_sc_address();
        let payment = EsdtTokenPayment::new(self.asset_id().get(), 0, amount);

        let stake_result = self.tx()
            .to(farm_address)
            .typed(proxy::FarmStakingProxy)
            .unstake_farm_endpoint(contract_address)
            .payment(payment)
            .returns(ReturnsResult)
            .sync_call()
            .into_tuple();

        self.last_stake_result().set(stake_result);
    }

    // ========== VIEWS ==========

    #[view(getBalanceOfAsset)]
    fn balance_of_asset(&self) -> BigUint {
        self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(self.asset_id().get()), 0)
    }

    #[view(getBalanceOfFarm)]
    fn balance_of_farm(&self) -> BigUint {
        let contract_address = self.blockchain().get_sc_address();
        let farm_address = self.farm_address().get();

        self.tx()
            .to(farm_address)
            .typed(proxy::FarmStakingProxy)
            .user_total_farm_position(contract_address)
            .returns(ReturnsResult)
            .sync_call()
    }

    #[view(getBalanceOf)]
    fn balance_of(&self) -> BigUint {
        self.balance_of_asset() + self.balance_of_farm()
    }

    // ========== STORAGE ==========
    
    #[view(getAssetId)]
    #[storage_mapper("assetId")]
    fn asset_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getFarmAddress)]
    #[storage_mapper("farmAddress")]
    fn farm_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getLastStakeResult)]
    #[storage_mapper("lastStakeResult")]
    fn last_stake_result(&self) -> SingleValueMapper<(EsdtTokenPayment, EsdtTokenPayment)>;
}
