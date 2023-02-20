//! Autogenerated weights for hospitals
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=hospitals
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/hospitals/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for hospitals.
pub trait WeightInfo {
	fn register_hospital() -> Weight;
	fn update_hospital() -> Weight;
	fn deregister_hospital() -> Weight;
}

/// Weights for hospitals using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCount (r:1 w:1)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:1 w:1)
	// Storage: UserProfile ProfileRolesByAccountId (r:1 w:1)
	fn register_hospital() -> Weight {
		Weight::from_ref_time(122_879_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:2 w:2)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:2 w:2)
	fn update_hospital() -> Weight {
		Weight::from_ref_time(154_341_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCount (r:1 w:1)
	fn deregister_hospital() -> Weight {
		Weight::from_ref_time(307_679_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCount (r:1 w:1)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:1 w:1)
	// Storage: UserProfile ProfileRolesByAccountId (r:1 w:1)
	fn register_hospital() -> Weight {
		Weight::from_ref_time(122_879_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:2 w:2)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:2 w:2)
	fn update_hospital() -> Weight {
		Weight::from_ref_time(154_341_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: Hospitals Hospitals (r:1 w:1)
	// Storage: Hospitals HospitalsByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCountByCountryRegionCity (r:1 w:1)
	// Storage: Hospitals HospitalCount (r:1 w:1)
	fn deregister_hospital() -> Weight {
		Weight::from_ref_time(307_679_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
