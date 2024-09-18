// Allow `cargo stylus export-abi` to generate a main function.
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
  alloy_primitives::U256,
  prelude::*,
  storage::StorageU256,
};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
#[storage]
#[entrypoint]
pub struct Counter {
    number: StorageU256,
}


/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[motsu::test]
    fn it_gets_number(contract: Counter) {
        let number = contract.number();
        assert_eq!(U256::ZERO, number);
    }

    #[motsu::test]
    fn it_sets_number(contract: Counter) {
        contract.set_number(U256::from(5));
        let number = contract.number();
        assert_eq!(U256::from(5), number);
    }

    #[motsu::test]
    fn it_multiplies(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.mul_number(U256::from(2));
        let number = contract.number();
        assert_eq!(U256::from(10), number);
    }

    #[motsu::test]
    fn it_adds(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.add_number(U256::from(2));
        let number = contract.number();
        assert_eq!(U256::from(7), number);
    }

    #[motsu::test]
    fn it_increments(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.increment();
        let number = contract.number();
        assert_eq!(U256::from(6), number);
    }
}