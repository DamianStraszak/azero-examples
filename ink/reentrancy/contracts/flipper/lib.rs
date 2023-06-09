#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use flipper::traits::flipper::*;
    use ink::{codegen::Env, env::CallFlags};
    use openbrush::{
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyFlipper {
        #[storage_field]
        guard: reentrancy_guard::Data,
        value: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Flipper for MyFlipper {
        #[ink(message)]
        fn get_value(&self) -> bool {
            self.value
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        fn flip(&mut self, num: u32) -> Result<(), ReentrancyGuardError> {
            if num<=1 {
                self.value = !self.value;
                return Ok(());
            }
            let own_address = self.env().account_id();
            flipper::traits::flipper::FlipperRef::flip_builder(&own_address,num-1).call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()?;
            Ok(())
        }
    }
}
