pub use openbrush::contracts::reentrancy_guard::*;

#[openbrush::wrapper]
pub type FlipperRef = dyn Flipper;

#[openbrush::trait_definition]
pub trait Flipper {
    #[ink(message)]
    fn get_value(&self) -> u32;

    #[ink(message)]
    fn increment_recursive(&mut self, num: u32) -> Result<(), ReentrancyGuardError>;

    #[ink(message)]
    fn increment_recursive_reentrant(&mut self, num: u32);
}
