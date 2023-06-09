#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use flipper::traits::flipper::*;
    use ink::{codegen::Env, env::CallFlags};
    use openbrush::traits::Storage;

    use ink::primitives::Key;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyFlipper {
        #[storage_field]
        guard: reentrancy_guard::Data,
        value: u32,
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
    }

    #[ink(event)]
    pub struct StorageDecodedAsGuard {
        status: u8,
    }

    #[ink(event)]
    pub struct StorageDecodedCorrectly {
        value: u32,
    }

    impl Flipper for MyFlipper {
        #[ink(message)]
        fn get_value(&self) -> u32 {
            let a: MyFlipper = ink::env::get_contract_storage::<Key, MyFlipper>(&0)
                .unwrap()
                .unwrap();
            a.value
        }

        

        #[ink(message)]
        fn increment_recursive_reentrant(&mut self, num: u32) {
            self.check_storage();

            if num == 0 {
                self.value += 1;
                return;
            }
            let own_address = self.env().account_id();
            flipper::traits::flipper::FlipperRef::increment_recursive_reentrant_builder(
                &own_address,
                num - 1,
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke();
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        fn increment_recursive(&mut self, num: u32) -> Result<(), ReentrancyGuardError> {
            self.check_storage();

            if num == 0 {
                self.value += 1;
                return Ok(());
            }
            let own_address = self.env().account_id();
            flipper::traits::flipper::FlipperRef::increment_recursive_builder(
                &own_address,
                num - 1,
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()?;
            Ok(())
        }
    }
}
