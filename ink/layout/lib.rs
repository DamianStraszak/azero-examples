#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod layout {
    use openbrush::{
        contracts::{
            psp22::*,
            reentrancy_guard::*,
            traits::psp22::PSP22Ref,
        },
        modifiers,
    };
    #[ink(storage)]
    pub struct Layout {
        value_0_32: u32,
    }


    impl Layout {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value_0_32: 11,
            }
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        pub fn init(&mut self) {

        }
    }
}
