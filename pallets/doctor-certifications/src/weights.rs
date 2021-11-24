//! Autogenerated weights for doctor_certifications
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-24, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=doctor-certifications
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/doctor-certifications/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for doctor_certifications.
pub trait WeightInfo {
	fn create_certification() -> Weight;
	fn update_certification() -> Weight;
	fn delete_certification() -> Weight;
}

/// Weights for doctor_certifications using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertifications (r:0 w:1)
	fn create_certification() -> Weight {
		(152_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	fn update_certification() -> Weight {
		(63_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	fn delete_certification() -> Weight {
		(113_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertifications (r:0 w:1)
	fn create_certification() -> Weight {
		(152_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	fn update_certification() -> Weight {
		(63_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	fn delete_certification() -> Weight {
		(113_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
