use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    user_accounts: LookupMap<AccountId, u128>,
    total_supply: u128,
}

impl Default for FungibleToken {
    fn default() -> FungibleToken {
        let mut contract = FungibleToken {
            user_accounts: LookupMap::new(b'm'),
            total_supply: 100,
        };

        let account_id = env::signer_account_id();

        contract
            .user_accounts
            .insert(&account_id, &contract.total_supply);

        return contract;
    }
}

#[near_bindgen]
impl FungibleToken {
    pub fn get_total_supply(&self) -> u128 {
        self.total_supply.clone()
    }

    pub fn get_balance_of(&self, account_id: AccountId) -> u128 {
        if let None = self.user_accounts.get(&account_id) {
            return 0;
        }

        return self.user_accounts.get(&account_id).unwrap();
    }

    pub fn transfer(&mut self, receiver_id: AccountId, tokens: u128) {
        let sender_id = env::signer_account_id();
        let initial_sender_amount;
        let initial_reciever_amount;

        if let None = self.user_accounts.get(&sender_id) {
            initial_sender_amount = 0
        } else {
            initial_sender_amount = self.user_accounts.get(&sender_id).unwrap();
        }

        assert!(initial_sender_amount >= tokens, "Not enough tokens!");
        self.user_accounts
            .insert(&sender_id, &(initial_sender_amount - tokens));

        if let None = self.user_accounts.get(&receiver_id) {
            initial_reciever_amount = 0
        } else {
            initial_reciever_amount = self.user_accounts.get(&receiver_id).unwrap();
        }

        self.user_accounts
            .insert(&receiver_id, &(initial_reciever_amount + tokens));
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn get_total_supply() {
        let contract = FungibleToken::default();
        assert_eq!(contract.get_total_supply(), 100)
    }

    #[test]
    fn get_balance_of() {
        let contract = FungibleToken::default();
        assert_eq!(contract.get_balance_of(env::signer_account_id()), 100);
        assert_eq!(
            contract.get_balance_of("emilbob.testnet".parse().unwrap()),
            0
        )
    }

    #[test]
    fn transfer_token() {
        let mut contract = FungibleToken::default();
        contract.transfer("emilbob.testnet".parse().unwrap(), 10);
        assert_eq!(contract.get_balance_of(env::signer_account_id()), 90);
        assert_eq!(
            contract.get_balance_of("emilbob.testnet".parse().unwrap()),
            10
        );
    }
}
