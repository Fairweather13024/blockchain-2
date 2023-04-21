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
    
    
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink::primitives::{
            Clear,
            Hash,
        };

        type Event = <IOU as ::ink::reflect::ContractEventBase>::Type;

        fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"IOU::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IOU::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IOU::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IOU::Transfer::value",
                    value: &expected_value,
                }),
            ];

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::CLEAR_HASH;
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {n}"
                );
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _iou = IOU::new(100, AccountId::from([0x01; 32]), 20);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }
               /// For calculating the event topic hash.
        struct PrefixedValue<'a, 'b, T> {
            pub prefix: &'a [u8],
            pub value: &'b T,
        }

        impl<X> scale::Encode for PrefixedValue<'_, '_, X>
        where
            X: scale::Encode,
        {
            #[inline]
            fn size_hint(&self) -> usize {
                self.prefix.size_hint() + self.value.size_hint()
            }

            #[inline]
            fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
                self.prefix.encode_to(dest);
                self.value.encode_to(dest);
            }
        }

        fn encoded_into_hash<T>(entity: &T) -> Hash
        where
            T: scale::Encode,
        {
            use ink::{
                env::hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                primitives::Clear,
            };

            let mut result = Hash::CLEAR_HASH;
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
}
}