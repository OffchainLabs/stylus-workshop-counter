// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

/// The solidity_storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[solidity_storage]
/// The entrypoint macro defines where Stylus execution begins. External methods
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct Counter {
  count: StorageU256,
}

#[external]
impl Counter {
  /// Gets the number from storage.
  pub fn get(&self) -> Result<U256, Vec<u8>> {
    Ok(self.count.get())
  }

  /// Sets the count in storage to a user-specified value.
  pub fn set_count(&mut self, count: U256) -> Result<(), Vec<u8>> {
    self.count.set(count);
    Ok(())
  }

  /// Increments count by 1
  pub fn increment(&mut self) -> Result<(), Vec<u8>> {
    let count = self.count.get() + U256::from(1);
    self.set_count(count)
  }
}
