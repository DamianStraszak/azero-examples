pub use openbrush::contracts::reentrancy_guard::*;

#[openbrush::wrapper]
pub type FlipperRef = dyn Flipper;

#[openbrush::trait_definition]
pub trait Flipper {
    #[ink(message)]
    fn get_value(&self) -> bool;

    #[ink(message)]
    fn flip(&mut self, num: u32) -> Result<(), ReentrancyGuardError>;
}
