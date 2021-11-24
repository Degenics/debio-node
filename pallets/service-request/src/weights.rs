//! Autogenerated weights for service_request
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-24, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=service-request
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/service-request/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for service_request.
pub trait WeightInfo {
	fn create_request() -> Weight;
	fn claim_request() -> Weight;
	fn process_request() -> Weight;
	fn finalize_request() -> Weight;
}

/// Weights for service_request using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ServiceRequest RequestByAccountId (r:1 w:1)
	// Storage: ServiceRequest ServiceCountRequest (r:1 w:1)
	// Storage: ServiceRequest RequestById (r:0 w:1)
	fn create_request() -> Weight {
		(208_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: Labs Labs (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest ServiceOfferById (r:0 w:1)
	fn claim_request() -> Weight {
		(87_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: ServiceRequest ServiceOfferById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest ServiceInvoiceById (r:0 w:1)
	// Storage: ServiceRequest ServiceInvoiceByOrderId (r:0 w:1)
	fn process_request() -> Weight {
		(176_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ServiceRequest AdminKey (r:1 w:0)
	// Storage: ServiceRequest ServiceInvoiceById (r:1 w:0)
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest RequestByAccountId (r:1 w:1)
	// Storage: ServiceRequest ServiceCountRequest (r:1 w:1)
	fn finalize_request() -> Weight {
		(236_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ServiceRequest RequestByAccountId (r:1 w:1)
	// Storage: ServiceRequest ServiceCountRequest (r:1 w:1)
	// Storage: ServiceRequest RequestById (r:0 w:1)
	fn create_request() -> Weight {
		(208_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: Labs Labs (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest ServiceOfferById (r:0 w:1)
	fn claim_request() -> Weight {
		(87_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: ServiceRequest ServiceOfferById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest ServiceInvoiceById (r:0 w:1)
	// Storage: ServiceRequest ServiceInvoiceByOrderId (r:0 w:1)
	fn process_request() -> Weight {
		(176_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ServiceRequest AdminKey (r:1 w:0)
	// Storage: ServiceRequest ServiceInvoiceById (r:1 w:0)
	// Storage: ServiceRequest RequestById (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ServiceRequest RequestByAccountId (r:1 w:1)
	// Storage: ServiceRequest ServiceCountRequest (r:1 w:1)
	fn finalize_request() -> Weight {
		(236_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}
