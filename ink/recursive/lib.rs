#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod traits {
    #[ink::trait_definition]
    pub trait RecursiveCall {
        #[ink(message, selector = 1)]
        fn get(&self) -> u64;

        #[ink(message, selector = 2)]
        fn increment(&mut self);

        #[ink(message, selector = 3)]
        fn increment_recursive(&mut self);
    }
}

#[ink::contract]
mod recursive {

    use crate::traits::RecursiveCall;
    use ink::env::{
        call::{ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct Recursive {
        value: u64,
    }

    #[ink(event)]
    pub struct CurrentValue {
        status: u64,
    }
    impl RecursiveCall for Recursive {
        #[ink(message, selector = 1)]
        fn get(&self) -> u64 {
            self.value
        }

        #[ink(message, selector = 2)]
        fn increment(&mut self) {
            if self.value > 0 {
                self.value = 10;
            }
            self.env().emit_event(CurrentValue { status: self.value });
        }

        #[ink(message, selector = 3)]
        fn increment_recursive(&mut self) {
            self.env().emit_event(CurrentValue { status: self.value });
            // let mut rec_ref: ink::contract_ref!(RecursiveCall) = (self.env().account_id()).into();
            // rec_ref.increment();
            self.value += 1;
            ink::env::call::build_call::<DefaultEnvironment>()
                .call(self.env().account_id())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .exec_input(ExecutionInput::new(Selector::new([0, 0, 0, 1])))
                .returns::<()>()
                .invoke();
            self.env().emit_event(CurrentValue { status: self.value });
        }
    }

    impl Recursive {
        #[ink(constructor)]
        pub fn new(init_value: u64) -> Self {
            Self { value: init_value }
        }
    }
}
