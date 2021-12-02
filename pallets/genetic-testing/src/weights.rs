//! Autogenerated weights for genetic_testing
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-01, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic-testing
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/genetic-testing/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_testing.
pub trait WeightInfo {
	fn reject_dna_sample() -> Weight;
	fn process_dna_sample() -> Weight;
	fn submit_test_result() -> Weight;
	fn submit_independent_test_result() -> Weight;
	fn submit_data_bounty_details() -> Weight;
}

/// Weights for genetic_testing using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	fn reject_dna_sample() -> Weight {
		(65_438_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn process_dna_sample() -> Weight {
		(17_531_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticTesting DnaTestResultsByLab (r:1 w:1)
	// Storage: GeneticTesting DnaTestResultsByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaTestResults (r:0 w:1)
	fn submit_test_result() -> Weight {
		(57_980_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticTesting DnaTestResults (r:1 w:1)
	// Storage: GeneticTesting DnaTestResultsByOwner (r:1 w:1)
	fn submit_independent_test_result() -> Weight {
		(54_853_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: GeneticTesting StakedDataByOrderId (r:0 w:1)
	// Storage: GeneticTesting StakedDataByAccountId (r:0 w:1)
	fn submit_data_bounty_details() -> Weight {
		(23_762_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	fn reject_dna_sample() -> Weight {
		(65_438_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn process_dna_sample() -> Weight {
		(17_531_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticTesting DnaTestResultsByLab (r:1 w:1)
	// Storage: GeneticTesting DnaTestResultsByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaTestResults (r:0 w:1)
	fn submit_test_result() -> Weight {
		(57_980_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: GeneticTesting DnaTestResults (r:1 w:1)
	// Storage: GeneticTesting DnaTestResultsByOwner (r:1 w:1)
	fn submit_independent_test_result() -> Weight {
		(54_853_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: GeneticTesting StakedDataByOrderId (r:0 w:1)
	// Storage: GeneticTesting StakedDataByAccountId (r:0 w:1)
	fn submit_data_bounty_details() -> Weight {
		(23_762_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
