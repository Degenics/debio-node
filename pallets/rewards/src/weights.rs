//! Autogenerated weights for rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-05, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=rewards
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/rewards/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for rewards.
pub trait WeightInfo {
	fn reward_funds() -> Weight;
	fn update_admin_key() -> Weight;
}

/// Weights for rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Rewards PalletAccount (r:1 w:0)
	// Storage: Rewards RewarderKey (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Rewards TotalRewardAmount (r:0 w:1)
	fn reward_funds() -> Weight {
		Weight::from_ref_time(35_358_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Rewards RewarderKey (r:1 w:1)
	fn update_admin_key() -> Weight {
		Weight::from_ref_time(23_276_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Rewards PalletAccount (r:1 w:0)
	// Storage: Rewards RewarderKey (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Rewards TotalRewardAmount (r:0 w:1)
	fn reward_funds() -> Weight {
		Weight::from_ref_time(35_358_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: Rewards RewarderKey (r:1 w:1)
	fn update_admin_key() -> Weight {
		Weight::from_ref_time(23_276_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
