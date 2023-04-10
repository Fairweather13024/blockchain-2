#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod iou_smart_contract {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Iou {
        total_supply: Balance,// Total token supply
        balances: Mapping<AccountId, Balance>,// Mapping from owner to number of owned token
        allowances: Mapping<(AccountId, AccountId), Balance>,
        due_date: u64,               // the date the IOU is due
        issuer: AccountId,           // the account ID of the IOU issuer
        recipient: AccountId,        // the account ID of the IOU recipient
        paid: bool,                  // whether the IOU has been paid or not
        partial_payment_amount: u32, // the amount of partial payment made on the IOU
    }

    //token transfer 
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    //approval occurs that `spender` is allowed to withdraw up to the amount of `value` tokens from `owner`
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        //insufficient balance
        InsufficientBalance,
        //insufficient allowance
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Iou {
        //Creates a new ERC-20 contract with the specified initial supply
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default(); //initialize balances mapping
            let caller = Self::env().caller(); // add total balance to account of person who deployed the contract
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        // allow a `caller` to add `value` tokens to the `caller`'s balance
        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let balance = self.balances.entry(caller).or_insert(0);
            *balance += amount;
            self.total_supply.set(self.total_supply.get() + amount);
        }

        //get total contract token supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        //check balance of `owner
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        //account balance for owner
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        //amount which `spender` is still allowed to withdraw from `owner`
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        //amount which `spender` is still allowed to withdraw from `owner`
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        #[ink(message)]
        pub fn transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }
    }
}
// #![cfg_attr(not(feature = "std"), no_std)]

// #[ink::contract]
// mod iou {

//     /// Defines the storage of your contract.
//     /// Add new fields to the below struct in order
//     /// to add new static storage fields to your contract.
//     #[ink(storage)]
//     pub struct Iou {
//         /// Stores a single `bool` value on the storage.
//         value: bool,
//     }

//     impl Iou {
//         /// Constructor that initializes the `bool` value to the given `init_value`.
//         #[ink(constructor)]
//         pub fn new(init_value: bool) -> Self {
//             Self { value: init_value }
//         }

//         /// Constructor that initializes the `bool` value to `false`.
//         ///
//         /// Constructors can delegate to other constructors.
//         #[ink(constructor)]
//         pub fn default() -> Self {
//             Self::new(Default::default())
//         }

//         /// A message that can be called on instantiated contracts.
//         /// This one flips the value of the stored `bool` from `true`
//         /// to `false` and vice versa.
//         #[ink(message)]
//         pub fn flip(&mut self) {
//             self.value = !self.value;
//         }

//         /// Simply returns the current value of our `bool`.
//         #[ink(message)]
//         pub fn get(&self) -> bool {
//             self.value
//         }
//     }

//     /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
//     /// module and test functions are marked with a `#[test]` attribute.
//     /// The below code is technically just normal Rust code.
//     #[cfg(test)]
//     mod tests {
//         /// Imports all the definitions from the outer scope so we can use them here.
//         use super::*;

//         /// We test if the default constructor does its job.
//         #[ink::test]
//         fn default_works() {
//             let iou = Iou::default();
//             assert_eq!(iou.get(), false);
//         }

//         /// We test a simple use case of our contract.
//         #[ink::test]
//         fn it_works() {
//             let mut iou = Iou::new(false);
//             assert_eq!(iou.get(), false);
//             iou.flip();
//             assert_eq!(iou.get(), true);
//         }
//     }
// }
