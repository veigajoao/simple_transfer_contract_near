use std::convert::TryInto;
use std::convert::TryFrom;

use near_sdk::{ borsh };
use borsh::{ BorshDeserialize, BorshSerialize };
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise,
    collections::{ LookupMap },
    json_types::{ U128 },
    utils::assert_one_yocto, ext_contract
};
use near_sdk::Gas;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

// const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
pub const FRACTIONAL_BASE: u128 = 10_000;
pub const BASE_GAS: Gas = 5_000_000_000_000;

#[ext_contract(token_contract)]
trait FungibleToken {
    fn ft_transfer(receiver_id: String, amount: String, memo: String);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PeterBot {
    
    pub owner_id: AccountId,
    pub transfer_fee: u128 // X / 10000

}

impl Default for PeterBot {
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl PeterBot {
    #[init]
    pub fn new(owner_id: AccountId, transfer_fee: U128) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id,
            transfer_fee: transfer_fee.0
        }
    }

    #[payable]
    pub fn transfer_payment(&mut self, receiver: AccountId) -> Promise {
        let deposit = env::attached_deposit();
        let receiver_amount = ( deposit * (FRACTIONAL_BASE - self.transfer_fee) ) / FRACTIONAL_BASE;
        Promise::new(receiver).transfer(receiver_amount)
    }

    pub fn ft_on_transfer(&mut self, sender_id: String, amount: String, msg: String) -> String {
        let receiver_amount = ( amount.parse::<u128>().unwrap() * (FRACTIONAL_BASE - self.transfer_fee) ) / FRACTIONAL_BASE;
        let receiver = msg;
        token_contract::ft_transfer(receiver, receiver_amount.to_string(), "penis".to_string(), 
                            &env::predecessor_account_id(), 0, BASE_GAS);
        "0".to_string()
    }
    
    #[payable]
    pub fn change_fee(&mut self, new_fee: U128) -> U128 {
        assert!(env::predecessor_account_id() == self.owner_id, "Only owner can call this function");
        assert_one_yocto();
        self.transfer_fee = new_fee.0;
        U128(self.transfer_fee)
    }

    pub fn get_fee(&self) -> U128 {
        U128(self.transfer_fee)
    }

    #[payable]
    pub fn withdraw_funds(&mut self, quantity: U128) -> Promise {
        assert!(env::predecessor_account_id() == self.owner_id, "Only owner can call this function");
        assert_one_yocto();
        Promise::new(self.owner_id.clone()).transfer(quantity.0)
    }

}