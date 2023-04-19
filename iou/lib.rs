#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod iou {
    use ink::storage::Mapping;
    
    #[ink(storage)]
    pub struct IOU {
        total_supply: Balance,  // Total token supply
        issuer: AccountId,                                    // the account ID of the IOU issuer
        recipient: AccountId,                                 // the account ID of the IOU recipient
        balances: Mapping<AccountId, Balance>, // Mapping from owner to number of owned token
        allowances: Mapping<(AccountId, AccountId), Balance>, // stores the amount that the owing account is still allowed to owe
        paid: bool,                  // whether the IOU has been paid or not
        partial_payment_amount: u32, // the amount of partial payment made on the IOU
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    impl IOU {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, recipient: AccountId, partial_payment_amount: u32) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            
            
            //transfer contract tokens to a smart contract address
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Default::default(),
                issuer: Self::env().caller(),
                recipient,
                paid: false,
                partial_payment_amount,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
    }
}
