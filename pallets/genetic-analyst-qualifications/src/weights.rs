//! Autogenerated weights for genetic_analyst_qualifications
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-07, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic-analyst-qualifications
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/genetic-analyst-qualifications/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_analyst_qualifications.
pub trait WeightInfo {
	fn create_qualification() -> Weight;
	fn update_qualification() -> Weight;
	fn delete_qualification() -> Weight;
}

/// Weights for genetic_analyst_qualifications using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCountByOwner (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCount (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:0 w:1)
	fn create_qualification() -> Weight {
		57_734_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:1 w:1)
	fn update_qualification() -> Weight {
		36_088_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:1 w:1)
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCount (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCountByOwner (r:1 w:1)
	fn delete_qualification() -> Weight {
		74_426_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCountByOwner (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCount (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:0 w:1)
	fn create_qualification() -> Weight {
		57_734_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:1 w:1)
	fn update_qualification() -> Weight {
		36_088_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: GeneticAnalystQualifications GeneticAnalystQualifications (r:1 w:1)
	// Storage: GeneticAnalysts GeneticAnalysts (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCount (r:1 w:1)
	// Storage: GeneticAnalystQualifications GeneticAnalystQualificationsCountByOwner (r:1 w:1)
	fn delete_qualification() -> Weight {
		74_426_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
