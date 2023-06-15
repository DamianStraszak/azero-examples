#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod lazytest {
    use ink::storage::{Lazy, traits::ManualKey};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct Lazytest {
        /// Stores a single `bool` value on the storage.
        value1: Lazy<u32>,
        value2: Lazy<u32, ManualKey<123>>,
    }

    impl Lazytest {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Lazytest::default()
        }


        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn set_1(&mut self, a: u32) {
            self.value1.set(&a);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_1(&self) -> Option<u32> {
            self.value1.get()
        }

        #[ink(message)]
        pub fn set_2(&mut self, a: u32) {
            self.value2.set(&a);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_2(&self) -> Option<u32> {
            self.value2.get()
        }
    }
}
