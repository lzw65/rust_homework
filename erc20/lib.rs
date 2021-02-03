#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        total_supplys: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
    }
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);
            let instance = Self {
                total_supplys: total_supply,
                balances: balances,
                allowance: StorageHashMap::new(),
            };
            instance
        }



        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            (*self).total_supplys
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();
            self.transfer_help(who, to, value)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let allowance_balance = self.allowance_of(from, to);
            if allowance_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.allowance.insert((from, to), allowance_balance - value);

            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: from,
                to: to,
                value: value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, owner: AccountId, value: Balance) ->  Result<()> {
            let balance = self.balance_of(owner);
            if balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.total_supplys = self.total_supply() - value;
            self.transfer_help(owner, AccountId::default(), value)
        }

        #[ink(message)]
        pub fn issue(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();
            self.allowance.insert((who, spender), value);
            Ok(())
        }

        fn transfer_help(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: from,
                to: to,
                value: value,
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let erc20 = Erc20::default();
            assert_eq!(erc20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }
}