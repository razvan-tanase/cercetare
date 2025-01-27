#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod strategy;

#[multiversx_sc::contract]
pub trait Vault: strategy::Strategy {
    #[init]
    fn init(&self, vault_token: TokenIdentifier, lp_pool_address: ManagedAddress) {
        require!(vault_token.is_valid_esdt_identifier(), "Invalid token identifier");
        self.vault_token().set(vault_token);

        self.set_lp_pool_address(lp_pool_address);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ==========

    // ========== INTERNALS ==========

    // ========== VIEWS ==========

    #[view(getRewardTokenIdentifier)]
    fn get_reward_token_identifier(&self) -> TokenIdentifier {
        self.get_lp_token_identifier()
    }
    
    #[view(getBalance)]
    fn get_balance(&self) -> BigUint {
        self.get_available() // + strategy().balance()
    }
    
    #[view(getAvailable)]
    fn get_available(&self) -> BigUint {
        self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(self.get_reward_token_identifier()), 0
        )
    }

    #[view(getPricePerFullShare)]
    fn get_price_per_full_share(&self) -> BigUint {
        let total_supply = self.total_supply().get();
        if total_supply == 0 {
            return BigUint::from(1u8);
        }

        self.get_balance() * BigUint::from(1u8).pow(18) / total_supply
    }

    // ========== STORAGE ==========
    
    #[storage_mapper("vault_token")]
    fn vault_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStrategy)]
    #[storage_mapper("strategy")]
    fn strategy(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTotalSupply)]
    #[storage_mapper("total_supply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;
}
