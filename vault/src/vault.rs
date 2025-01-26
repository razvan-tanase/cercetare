#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod strategy;

#[multiversx_sc::contract]
pub trait Vault: strategy::Strategy {
    #[init]
    fn init(&self, vault_token: TokenIdentifier) {
        require!(vault_token.is_valid_esdt_identifier(), "Invalid token identifier");
        self.vault_token().set(vault_token);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ==========

    // ========== INTERNALS ==========

    // ========== VIEWS ==========

        // Function: want()
        #[view(want)]
        fn want(&self) -> EgldOrEsdtTokenIdentifier {
            self.want_token().get()
        }
    
        // Function: balance()
        #[view(balance)]
        fn balance(&self) -> BigUint {
            let contract_address = self.blockchain().get_sc_address();
    
            // Balance of the vault contract
            let vault_balance = self.blockchain().get_sc_balance(&self.want());
    
            // Balance held by the strategy contract
            let strategy_address = self.strategy().get();
            let strategy_balance: BigUint = self
                .blockchain()
                .query_contract::<BigUint>(&strategy_address, "balance_of", &[]);
    
            vault_balance + strategy_balance
        }
    
        #[view(available)]
        fn available(&self) -> BigUint {
            // Balance of the vault contract
            self.blockchain().get_sc_balance(&self.want())
        }

    // ========== STORAGE ==========
    
    #[storage_mapper("strategy")]
    fn strategy(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("vault_token")]
    fn vault_token(&self) -> SingleValueMapper<TokenIdentifier>;
}
