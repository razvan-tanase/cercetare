#![no_std]

use multiversx_sc::imports::*;

pub mod proxy;

#[multiversx_sc::contract]
pub trait LiquidityAdder {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(addLiquidity)]
    fn add_liquidity(
        &self, 
        destination: ManagedAddress
    ) {
        let nr_required_transfers = 2;
        let multi_payment = self.call_value().all_esdt_transfers();

        require!(
            multi_payment.len() == nr_required_transfers,
            "Invalid number of transfers"
        );

        let first_transfer = multi_payment.get(0);
        let second_transfer = multi_payment.get(1);

        let first_transfer_amount = first_transfer.amount;
        let second_transfer_amount = second_transfer.amount;

        let first_token_amount_min = first_transfer_amount / BigUint::from(100u32);
        let second_token_amount_min = second_transfer_amount / BigUint::from(100u32);

        self.tx()
            .to(destination)
            .typed(proxy::PairProxy)
            .add_liquidity(
                first_token_amount_min, 
                second_token_amount_min
            )
            .payment(multi_payment)
            .sync_call();
    }

    #[view(getRewardTokenIdentifier)]
    fn get_reward_token_identifier(&self, destination: ManagedAddress) -> TokenIdentifier {
        let result = self.tx()
            .to(destination)
            .typed(proxy::PairProxy)
            .get_lp_token_identifier()
            .returns(ReturnsResult)
            .sync_call_readonly();
        
        result
    }
}
