#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod iou {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct IOU {
        amount_owed: Balance,                                      // Total token supply
        issuer: AccountId,                                    // the account ID of the IOU issuer
        recipient: AccountId,                                 // the account ID of the IOU recipient
        balances: Mapping<AccountId, Balance>, // Mapping from owner to number of owned token
        allowances: Mapping<(AccountId, AccountId), Balance>, // stores the amount that the owing account is still allowed to owe
        paid: bool,                       // whether the IOU has been paid or not
        partial_payment_percentage: u128, // the amount of partial payment made on the IOU
        amount_paid: u128,
    }

    //emit a transfer to the blockchain
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    //emit a supply contract instance to the blockchain
    #[ink(event)]
    pub struct IouContract {
        #[ink(topic)]
        issuer: Option<AccountId>,
        #[ink(topic)]
        recipient: Option<AccountId>,
        #[ink(topic)]
        amount_owed: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl IOU {

        //instantiate a new smart contract
        #[ink(constructor)]
        pub fn new(
            amount_owed: Balance,
            recipient: AccountId,
            partial_payment_percentage: u128,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &amount_owed);

            //increment amount_paid by the amount
            let amount_paid = 0;
            let amount_paid = amount_paid + amount_owed;

            //transfer contract tokens to the smart contract address
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: amount_owed,
            });

            //emit a iou contract instance 
            Self::env().emit_event(IouContract {
                issuer: Some(Self::env().caller()),
                recipient: Some(recipient),
                amount_owed: amount_owed,
            });

            //instantiate a new smart contract
            Self {
                amount_owed,
                balances,
                allowances: Default::default(),
                issuer: Self::env().caller(),
                recipient,
                paid: false,
                partial_payment_percentage,
                amount_paid,
            }
        }

        //check the amount that is still owed in this smart contract
        #[ink(message)]
        pub fn amount(&self) -> Balance {
            self.amount_owed
        }

        //allow a debtor to pay the issuer by an amount
        #[ink(message)]
        pub fn pay_debt(&mut self, amount: Balance) {
            self.amount_owed -= amount;
            let from = self.env().caller();
            let to = self.recipient;
            let value = amount;

            self.transfer_from_to(&from, &to, value);
        }
        
        #[ink(message)]
        pub fn public_account_balance(&self) -> Balance {
            self.balance_of_account(&self.env().caller())
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            //add tokens to one callers account
            let to = self.env().caller();
            let value = amount;

            let to_balance = self.balance_of_account(&to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Deposit {
                to: Some(to),
                value,
            });
        }

        fn balance_of_account(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        //allow the issuer to transfer tokens to the recipient
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_account(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }

            //decrease the balance of the issuer
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_account(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            //increase the allowance between the issuer and recipient by the amount paid
            self.allowances.insert((&from, &to), &value);
            Ok(())
        }
    }
}
