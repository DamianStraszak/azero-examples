#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use ink::primitives::Key;
    use openbrush::{
        contracts::reentrancy_guard::{self, non_reentrant, ReentrancyGuardError},
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyFlipper {
        value: u32,
        #[storage_field]
        guard: reentrancy_guard::Data,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                guard: reentrancy_guard::Data::default(),
                value: 1234567,
            }
        }

        fn check_storage(&self) {
            match ink::env::get_contract_storage::<Key, MyFlipper>(&0) {
                Ok(Some(s)) => {
                    self.env()
                        .emit_event(StorageDecodedCorrectly { value: s.value });
                }
                _ => {}
            };

            match ink::env::get_contract_storage::<Key, reentrancy_guard::Data>(&0) {
                Ok(Some(g)) => {
                    self.env()
                        .emit_event(StorageDecodedAsGuard { status: g.status });
                }
                _ => {}
            };
        }

        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn increment_reentrant(&mut self) {
            self.check_storage();
            self.value += 1;
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn increment(&mut self) -> Result<(), ReentrancyGuardError> {
            self.check_storage();
            self.value += 1;
            Ok(())
        }
    }

    #[ink(event)]
    pub struct StorageDecodedAsGuard {
        status: u8,
    }

    #[ink(event)]
    pub struct StorageDecodedCorrectly {
        value: u32,
    }
}
