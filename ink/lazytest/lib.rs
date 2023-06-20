#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod lazytest {
    use ink::storage::{traits::ManualKey, Lazy};

    #[derive(Default)]
    #[ink(storage)]
    pub struct Lazytest {
        value1: Lazy<u32>,
        value2: Lazy<u32, ManualKey<123>>,
    }

    impl Lazytest {
        #[ink(constructor)]
        pub fn new() -> Self {
            Lazytest::default()
        }
        #[ink(message)]
        pub fn set_1(&mut self, a: u32) {

            self.value1.set(&a);
        }

        #[ink(message)]
        pub fn get_1(&self) -> Option<u32> {
            self.value1.get()
        }

        #[ink(message)]
        pub fn set_2(&mut self, a: u32) {
            self.value2.set(&a);
        }

        #[ink(message)]
        pub fn get_2(&self) -> Option<u32> {
            self.value2.get()
        }
    }
}
