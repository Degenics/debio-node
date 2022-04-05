//! Autogenerated weights for orders
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
// --pallet=orders
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/orders/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for orders.
pub trait WeightInfo {
	fn create_order() -> Weight;
	fn cancel_order() -> Weight;
	fn set_order_paid() -> Weight;
	fn fulfill_order() -> Weight;
	fn set_order_refunded() -> Weight;
	fn update_escrow_key() -> Weight;
}

/// Weights for orders using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Services Services (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByLab (r:1 w:1)
	// Storage: Orders OrdersBySeller (r:1 w:1)
	// Storage: Orders PendingOrdersBySeller (r:1 w:1)
	// Storage: Orders OrdersByCustomer (r:1 w:1)
	// Storage: Orders Orders (r:0 w:1)
	// Storage: Orders LastOrderByCustomer (r:0 w:1)
	fn create_order() -> Weight {
		112_135_000_u64
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_order() -> Weight {
		52_015_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_order_paid() -> Weight {
		40_731_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn fulfill_order() -> Weight {
		47_454_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_order_refunded() -> Weight {
		51_900_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:1)
	fn update_escrow_key() -> Weight {
		22_233_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Services Services (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByLab (r:1 w:1)
	// Storage: Orders OrdersBySeller (r:1 w:1)
	// Storage: Orders PendingOrdersBySeller (r:1 w:1)
	// Storage: Orders OrdersByCustomer (r:1 w:1)
	// Storage: Orders Orders (r:0 w:1)
	// Storage: Orders LastOrderByCustomer (r:0 w:1)
	fn create_order() -> Weight {
		112_135_000_u64
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_order() -> Weight {
		52_015_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_order_paid() -> Weight {
		40_731_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn fulfill_order() -> Weight {
		47_454_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_order_refunded() -> Weight {
		51_900_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: Orders EscrowKey (r:1 w:1)
	fn update_escrow_key() -> Weight {
		22_233_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
