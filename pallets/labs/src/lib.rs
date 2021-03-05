#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, traits::Get,
    // debug
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{RuntimeDebug};
use frame_support::sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Lab<AccountId, Hash> {
    id: AccountId,
    name: Vec<u8>,
    services: Vec<Hash>,
    country: Vec<u8>,
    city: Vec<u8>,
    address: Vec<u8>,
    latitude: Option<Vec<u8>>,
    longitude: Option<Vec<u8>>,
    profile_image: Option<Vec<u8>>,
    is_verified: bool,
}

impl<AccountId, Hash> Lab<AccountId, Hash> {
    pub fn get_id(&self) -> &AccountId {
        &self.id
    }
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as LabsStorage {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
                Labs get(fn lab_by_account_id):
                    map hasher(blake2_128_concat) T::AccountId
                        => Option<Lab<T::AccountId, T::Hash>>;
                LabCount get(fn lab_count): u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
        where
            AccountId = <T as frame_system::Trait>::AccountId,
            Hash = <T as frame_system::Trait>::Hash,
        {
		/// Event documentation should end with an array that provides descriptive names for event
                /// parameters. [Lab, who]
                LabRegistered(Lab<AccountId, Hash>, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
                /// Account already has lab registered
                LabAlreadyRegistered,
                /// Lab identified by the AccountId does not exist
                LabDoesNotExist,
                /// Service does not exist
                ServiceDoesNotExist,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

                #[weight = 10_000 + T::DbWeight::get().writes(1)]
                pub fn register_lab(
                    origin,
                    lab_name: Vec<u8>,
                    country: Vec<u8>,
                    city: Vec<u8>,
                    address: Vec<u8>,
                    latitude: Option<Vec<u8>>,
                    longitude: Option<Vec<u8>>,
                    profile_image: Option<Vec<u8>>,
                    is_verified: Option<bool>
                ) -> dispatch::DispatchResult
                {
                    let who = ensure_signed(origin)?;

                    let lab_exists = <Labs<T>>::contains_key(&who);
                    if lab_exists {
                        return Err(Error::<T>::LabAlreadyRegistered)?;
                    }

                    let services: Vec<<T as frame_system::Trait>::Hash> = Vec::new();
                    let lab = Lab {
                        id: who.clone(),
                        name: lab_name,
                        country: country,
                        city: city,
                        address: address,
                        latitude: latitude,
                        longitude: longitude,
                        profile_image: profile_image,
                        services: services,
                        is_verified: is_verified.unwrap_or(false)
                    };
                    <Labs<T>>::insert(&who, &lab);

                    // Add lab count
                    let lab_count = LabCount::get();
                    LabCount::put(lab_count.wrapping_add(1));

                    Self::deposit_event(RawEvent::LabRegistered(lab, who.clone()));

                    Ok(())
                }

	}
}

impl<T: Trait> Module<T> {
    pub fn associate_service_to_lab(
        lab_id: &<T as frame_system::Trait>::AccountId,
        service_id: <T as frame_system::Trait>::Hash
    ) -> ()
    {
        <Labs<T>>::mutate(lab_id, | lab | {
            match lab {
                None => (), // If lab does not exist, do nothing
                Some(_lab) => {
                    _lab.services.push(service_id);
                }
            }
        });
    }

    // Reads the nonce from storage, increments the stored nonce, and returns
    // the encoded nonce to the caller.
    //fn encode_and_update_nonce() -> Vec<u8> {
    //	let nonce = Nonce::get();
    //	Nonce::put(nonce.wrapping_add(1));
    //	nonce.encode()
    //}
}
