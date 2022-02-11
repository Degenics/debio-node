//! Autogenerated weights for electronic_medical_record
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-11, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=electronic-medical-record
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/electronic-medical-record/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for electronic_medical_record.
pub trait WeightInfo {
	fn add_electronic_medical_record() -> Weight;
	fn update_electronic_medical_record() -> Weight;
	fn remove_electronic_medical_record() -> Weight;
}

/// Weights for electronic_medical_record using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:0 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:0 w:1)
	fn add_electronic_medical_record() -> Weight {
		74_150_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn update_electronic_medical_record() -> Weight {
		124_364_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	fn remove_electronic_medical_record() -> Weight {
		80_019_000_u64
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:0 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:0 w:1)
	fn add_electronic_medical_record() -> Weight {
		74_150_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn update_electronic_medical_record() -> Weight {
		124_364_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	fn remove_electronic_medical_record() -> Weight {
		80_019_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
