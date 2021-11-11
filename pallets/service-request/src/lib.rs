#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*,
	scale_info::TypeInfo,
	sp_std::prelude::*,
	sp_runtime::{RuntimeDebug, traits::{AccountIdConversion, Hash, Zero}},
	traits::{Currency, ExistenceRequirement, UnixTime, WithdrawReasons},
	PalletId,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use labs::interface::{LabInterface, LabVerificationStatusTrait};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod interface;
pub use interface::SeviceRequestInterface;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum RequestStatus {
	Open,
	WaitingForUnstaked,
	Unstaked,
	Claimed,
	Processed,
	Finalized,
}
impl Default for RequestStatus {
	fn default() -> Self {
		RequestStatus::Open
	}
}

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Request<AccountId, Balance, Hash> {
	pub hash: Hash,
	pub requester_address: AccountId,
	pub lab_address: Option<AccountId>,
	pub country: Vec<u8>,
	pub region: Vec<u8>,
	pub city: Vec<u8>,
	pub service_category: Vec<u8>,
	pub staking_amount: Balance,
	pub status: RequestStatus,
	pub unstaked_at: Option<u128>,
}
impl<AccountId, Balance, Hash> Request<AccountId, Balance, Hash> {
	pub fn new(
		hash: Hash,
		requester_address: AccountId,
		country: Vec<u8>,
		region: Vec<u8>,
		city: Vec<u8>,
		service_category: Vec<u8>,
		staking_amount: Balance,
	) -> Self {
		Self {
			hash,
			requester_address,
			lab_address: None,
			country,
			region,
			city,
			service_category,
			staking_amount,
            status: RequestStatus::default(),
            unstaked_at: None,
        }
    }
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceOffer<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub lab_address: AccountId,
	pub service_id: Hash,
	pub testing_price: Balance,
	pub qc_price: Balance,
}
impl<AccountId, Balance, Hash> ServiceOffer<AccountId, Balance, Hash> {
	pub fn new(
		request_hash: Hash,
		lab_address: AccountId,
		service_id: Hash,
		testing_price: Balance,
		qc_price: Balance,
	) -> Self {
		Self { request_hash, lab_address, service_id, testing_price, qc_price }
	}
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceInvoice<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub order_id: Hash,
	pub service_id: Hash,
	pub customer_address: AccountId,
	pub seller_address: AccountId,
	pub dna_sample_tracking_id: Vec<u8>,
	pub testing_price: Balance,
	pub qc_price: Balance,
	pub pay_amount: Balance,
}
impl<AccountId, Balance, Hash> ServiceInvoice<AccountId, Balance, Hash> {
	pub fn new(
		request_hash: Hash,
		order_id: Hash,
		service_id: Hash,
		customer_address: AccountId,
		seller_address: AccountId,
		dna_sample_tracking_id: Vec<u8>,
		testing_price: Balance,
		qc_price: Balance,
		pay_amount: Balance,
	) -> Self {
		Self {
			request_hash,
			order_id,
			service_id,
			customer_address,
			seller_address,
			dna_sample_tracking_id,
			testing_price,
			qc_price,
			pay_amount,
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	pub const PALLET_ID: PalletId = PalletId(*b"reqsrvc!");

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type AdminOf<T> = AccountIdOf<T>;
	pub type RequestOf<T> = Request<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
	pub type ServiceOfferOf<T> = ServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
	pub type ServiceInvoiceOf<T> = ServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
	pub type RequestIdOf<T> = HashOf<T>;
	pub type RequesterIdOf<T> = AccountIdOf<T>;
	pub type LabIdOf<T> = AccountIdOf<T>;
	pub type CountryOf = Vec<u8>;
	pub type RegionOf = Vec<u8>;
	pub type CityOf = Vec<u8>;
	pub type ServiceCategoryOf = Vec<u8>;
	pub type ServiceIdOf<T> = HashOf<T>;
	pub type OrderIdOf<T> = HashOf<T>;
	pub type DNASampleTrackingIdOf = Vec<u8>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Labs: LabInterface<Self>;
	}

	#[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub admin_key: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                admin_key: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            AdminKey::<T>::put(&self.admin_key);
        }
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ServiceRequestCreated(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestWaitingForUnstaked(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestUnstaked(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestWaitingForClaimed(AccountIdOf<T>, ServiceOfferOf<T>),
		ServiceRequestClaimed(AccountIdOf<T>, ServiceOfferOf<T>),
		ServiceRequestProcessed(AccountIdOf<T>, ServiceInvoiceOf<T>),
		ServiceRequestFinalized(AccountIdOf<T>, ServiceInvoiceOf<T>),
		StakingAmountRefunded(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
		StakingAmountExcessRefunded(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
		StakingAmountIncreased(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		BadSignature,
		UnAuthorized,
		NotValidAmount,
		RequestNotFound,
		RequestUnableToUnstake,
		RequestUnableToRetrieveUnstake,
		RequestUnableToProccess,
		RequestUnableToFinalize,
		RequestWaitingForUnstaked,
		RequestAlreadyUnstaked,
		RequestAlreadyInList,
		RequestAlreadyClaimed,
		RequestAlreadyProccessed,
		RequestAlreadyFinalized,
		ServiceOfferNotFound,
		ServiceInvoiceNotFound,
		LabNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	/// Get Request by RequestId
	#[pallet::storage]
	#[pallet::getter(fn request_by_id)]
	pub type RequestById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, RequestOf<T>>;

	/// Get RequestIds by LabId
	#[pallet::storage]
	#[pallet::getter(fn requests_by_lab_id)]
	pub type RequestsByLabId<T> = StorageMap<_, Blake2_128Concat, LabIdOf<T>, Vec<RequestIdOf<T>>, ValueQuery>;

	/// Get  ServiceCountRequest by Country, Region, City, ServiceCategoryOf
	#[pallet::storage]
	#[pallet::getter(fn service_count_request)]
	pub type ServiceCountRequest<T> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CountryOf>,
			NMapKey<Blake2_128Concat, RegionOf>,
			NMapKey<Blake2_128Concat, CityOf>,
			NMapKey<Blake2_128Concat, ServiceCategoryOf>,
		),
		u64,
		ValueQuery,
	>;

	/// Get ServiceOffer by RequestId
	#[pallet::storage]
	#[pallet::getter(fn service_offer_by_id)]
	pub type ServiceOfferById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, ServiceOfferOf<T>>;

	/// Get ServiceInvoice by RequestId
	#[pallet::storage]
	#[pallet::getter(fn service_invoice_by_id)]
	pub type ServiceInvoiceById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, ServiceInvoiceOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn create_request(
			origin: OriginFor<T>,
			country: CountryOf,
			region: RegionOf,
			city: CityOf,
			service_category: ServiceCategoryOf,
			staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::create_request(
				who.clone(),
				country.clone(),
				region.clone(),
				city.clone(),
				service_category.clone(),
				staking_amount.clone(),
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestCreated(who.clone(), request));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn unstake(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::unstake(
				who.clone(),
				request_id.clone(),
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestWaitingForUnstaked(who.clone(), request));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn retrieve_unstaked_amount(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::retrieve_unstaked_amount(
				who.clone(),
				request_id.clone(),
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUnstaked(who.clone(), request));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn claim_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
			service_id: HashOf<T>,
			testing_price: BalanceOf<T>,
			qc_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::claim_request(
				who.clone(),
				request_id.clone(),
				service_id.clone(),
				testing_price,
				qc_price,
			) {
				Ok((request, service_offer)) => {
					if request.status == RequestStatus::Claimed {
						Self::deposit_event(Event::ServiceRequestClaimed(who.clone(), service_offer));
					} else {
						Self::deposit_event(Event::ServiceRequestWaitingForClaimed(who.clone(), service_offer));
					}
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn process_request(
			origin: OriginFor<T>,
			lab_id: LabIdOf<T>,
			request_id: HashOf<T>,
			order_id: HashOf<T>,
			dna_sample_tracking_id: DNASampleTrackingIdOf,
			additional_staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::process_request(
				who.clone(),
				lab_id.clone(),
				request_id.clone(),
				order_id.clone(),
				dna_sample_tracking_id.clone(),
				additional_staking_amount.clone(),
			) {
				Ok(service_invoice) => {
					Self::deposit_event(Event::ServiceRequestProcessed(who.clone(), service_invoice));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn finalize_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
			test_result_success: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::finalize_request(
				who.clone(),
				request_id.clone(),
				test_result_success.clone(),
			) {
				Ok(service_invoice) => {
					Self::deposit_event(Event::ServiceRequestFinalized(who.clone(), service_invoice));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	pub fn staking_account_id(request_id: ServiceIdOf<T>) -> AccountIdOf<T> {
		PALLET_ID.into_sub_account(request_id)
	}
}

/// Service Request Interface Implementation
impl<T: Config> SeviceRequestInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Balance = BalanceOf<T>;
	type Request = RequestOf<T>;
	type Admin = AdminOf<T>;
	type ServiceOffer = ServiceOfferOf<T>;
	type ServiceInvoice = ServiceInvoiceOf<T>;
	type RequestId = RequestIdOf<T>;
	type RequesterId = RequesterIdOf<T>;
	type LabId = LabIdOf<T>;
	type Country = CountryOf;
	type Region = RegionOf;
	type City = CityOf;
	type ServiceCategory = ServiceCategoryOf;
	type ServiceId = ServiceIdOf<T>;
	type OrderId = OrderIdOf<T>;
	type DNASampleTrackingId = DNASampleTrackingIdOf;

	fn generate_request_id(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
	) -> Self::RequestId {
		let mut seed = requester_id.encode();
		let account_info = frame_system::Pallet::<T>::account(requester_id);

		seed.append(&mut account_info.nonce.encode());
		seed.append(&mut country.encode());
		seed.append(&mut region.encode());
		seed.append(&mut city.encode());
		seed.append(&mut service_category.encode());

		T::Hashing::hash(&seed)
	}

	fn create_request(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
		staking_amount: Self::Balance,
    ) -> Result<Self::Request, Self::Error> {
		if staking_amount.is_zero() {
            return Err(Error::<T>::NotValidAmount);
        }

		let request_id = Self::generate_request_id(
			requester_id.clone(),
			country.clone(),
			region.clone(),
			city.clone(),
			service_category.clone(),
		);

		match CurrencyOf::<T>::withdraw(
			&requester_id,
			staking_amount.clone(),
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(&Self::staking_account_id(request_id), imb);

				let request = Request::new(
					request_id.clone(),
					requester_id.clone(),
					country.clone(),
					region.clone(),
					city.clone(),
					service_category.clone(),
					staking_amount.clone(),
				);

				RequestById::<T>::insert(request_id.clone(), request.clone());

				let service_count = ServiceCountRequest::<T>::get((
					country.clone(),
					region.clone(),
					city.clone(),
					service_category.clone(),
				));

				ServiceCountRequest::<T>::insert(
					(country, region, city, service_category),
					service_count.wrapping_add(1),
				);

				Ok(request.clone())
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn unstake(
		requester_id: Self::RequesterId,
		request_id: Self::RequestId,
	) -> Result<Self::Request, Self::Error> {
		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if request.requester_address != requester_id {
			return Err(Error::<T>::UnAuthorized);
		}

		if request.status != RequestStatus::Open {
			return Err(Error::<T>::RequestUnableToUnstake);
		}

		request.status = RequestStatus::WaitingForUnstaked;
		let now: u128 = T::TimeProvider::now().as_millis();
		request.unstaked_at = Some(now);

		RequestById::<T>::insert(request_id, request.clone());

		Ok(request)
	}

	fn retrieve_unstaked_amount(
		admin: Self::Admin,
		request_id: Self::RequestId,
	) -> Result<Self::Request, Self::Error> {
		if admin.clone() != AdminKey::<T>::get() {
            return Err(Error::<T>::UnAuthorized);
        }

		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if request.status != RequestStatus::WaitingForUnstaked {
			return Err(Error::<T>::RequestUnableToRetrieveUnstake);
		}

		let requester_id = request.requester_address.clone();

		let now: u128 = T::TimeProvider::now().as_millis();
		let unstaked_at: u128 = request.unstaked_at.unwrap();
		// TODO: move to constant runtime config
		let six_days: u128 = 3600 as u128 * 144 as u128 * 1000 as u128;

		if (now - unstaked_at) == six_days {
			return Err(Error::<T>::RequestWaitingForUnstaked);
		}

		match CurrencyOf::<T>::withdraw(
			&Self::staking_account_id(request_id),
			request.staking_amount.clone(),
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(&requester_id, imb);

				Self::deposit_event(Event::StakingAmountRefunded(
					requester_id.clone(), request_id.clone(), request.staking_amount.clone()
				));

				request.status = RequestStatus::Unstaked;

				RequestById::<T>::insert(request_id, request.clone());

				Ok(request)
			},
			_ => Err(Error::<T>::BadSignature),
		}
	}

	fn claim_request(
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		service_id: Self::ServiceId,
		testing_price: Self::Balance,
		qc_price: Self::Balance,
	) -> Result<(Self::Request, Self::ServiceOffer), Self::Error> {
		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if request.status == RequestStatus::Claimed {
			return Err(Error::<T>::RequestAlreadyClaimed);
		}

		let lab_status = T::Labs::lab_verification_status(&lab_id);

		if lab_status.is_none() {
			return Err(Error::<T>::LabNotFound);
		}

		let lab_status = lab_status.unwrap();

		let service_offer = ServiceOffer::new(
			request_id.clone(),
			lab_id.clone(),
			service_id,
			testing_price,
			qc_price,
		);

		if lab_status.is_verified() {
			request.status = RequestStatus::Claimed;
			request.lab_address = Some(lab_id);

			RequestById::<T>::insert(request_id, request.clone());
			ServiceOfferById::<T>::insert(request_id, service_offer.clone());
		} else {
			let mut request_ids = RequestsByLabId::<T>::get(lab_id.clone());
			let found = request_ids.iter().find(|x| x == &&request_id);

			if found.is_some() {
				return Err(Error::<T>::RequestAlreadyInList);
			}

			request_ids.push(request_id.clone());

			RequestsByLabId::<T>::insert(lab_id.clone(), request_ids);
		}

		Ok((request, service_offer))
	}

	fn process_request(
		requester_id: Self::RequesterId,
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		order_id: Self::OrderId,
		dna_sample_tracking_id: Self::DNASampleTrackingId,
		additional_staking_amount: Self::Balance,
	) -> Result<Self::ServiceInvoice, Self::Error> {
		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if requester_id != request.requester_address {
			return Err(Error::<T>::UnAuthorized);
		}

		if request.lab_address.is_none() {
			return Err(Error::<T>::LabNotFound);
		}

		if request.status == RequestStatus::WaitingForUnstaked
			|| request.status == RequestStatus::Unstaked {
			return Err(Error::<T>::RequestAlreadyUnstaked);
		}

		if request.status != RequestStatus::Claimed {
			return Err(Error::<T>::RequestUnableToProccess);
		}

		let service_offer = ServiceOfferById::<T>::get(request_id.clone());

		if service_offer.is_none() {
			return Err(Error::<T>::ServiceOfferNotFound);
		}

		let service_offer = service_offer.unwrap();

		let pay_amount = request.staking_amount;
		let testing_price = service_offer.testing_price;
		let qc_price = service_offer.qc_price;
		let total_price = testing_price.clone() + qc_price.clone();
		let mut excess = Zero::zero();
		let mut final_pay_amount = pay_amount.clone();

		if pay_amount.clone() > total_price.clone() {
			excess = pay_amount.clone() - total_price.clone();
			final_pay_amount = total_price.clone();
		}

		if !excess.is_zero() {
			match CurrencyOf::<T>::withdraw(
				&Self::staking_account_id(request_id),
				excess.clone(),
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			) {
				Ok(imb) => {
					CurrencyOf::<T>::resolve_creating(&requester_id, imb);

					Self::deposit_event(Event::StakingAmountExcessRefunded(
						requester_id.clone(), request_id.clone(), excess.clone()
					));
				},
				_ => {
					return Err(Error::<T>::BadSignature)
				},
			}
		} else {
			final_pay_amount = request.staking_amount + additional_staking_amount;

			if final_pay_amount.clone() != total_price.clone() {
				return Err(Error::<T>::NotValidAmount)
			} else {
				match CurrencyOf::<T>::withdraw(
					&requester_id,
					additional_staking_amount.clone(),
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::KeepAlive,
				) {
					Ok(imb) => {
						CurrencyOf::<T>::resolve_creating(&Self::staking_account_id(request_id), imb);

						Self::deposit_event(Event::StakingAmountIncreased(
							requester_id.clone(), request_id.clone(), additional_staking_amount.clone()
						));
					},
					_ => {
						return Err(Error::<T>::BadSignature)
					},
				}
			}
		}

		let service_invoice = ServiceInvoice::new(
			request_id.clone(),
			order_id.clone(),
			service_offer.service_id.clone(),
			requester_id.clone(),
			lab_id.clone(),
			dna_sample_tracking_id.clone(),
			testing_price.clone(),
			qc_price.clone(),
			final_pay_amount.clone(),
		);

		ServiceInvoiceById::<T>::insert(request_id.clone(), service_invoice.clone());

		request.status = RequestStatus::Processed;
		RequestById::<T>::insert(request_id.clone(), request);

		Ok(service_invoice.clone())
	}

	fn finalize_request(
		admin: Self::Admin,
		request_id: Self::RequestId,
		test_result_success: bool,
	) -> Result<Self::ServiceInvoice, Self::Error> {
		if admin.clone() != AdminKey::<T>::get() {
            return Err(Error::<T>::UnAuthorized);
        }

		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		let service_offer = ServiceOfferById::<T>::get(request_id.clone());

		if service_offer.is_none() {
			return Err(Error::<T>::ServiceOfferNotFound);
		}

		let _service_offer = service_offer.unwrap();

		let service_invoice = ServiceInvoiceById::<T>::get(request_id.clone());

		if service_invoice.is_none() {
			return Err(Error::<T>::ServiceInvoiceNotFound);
		}

		let service_invoice = service_invoice.unwrap();

		let requester_id = service_invoice.customer_address.clone();
		let lab_id = service_invoice.seller_address.clone();

		if request.status != RequestStatus::Processed {
			return Err(Error::<T>::RequestUnableToFinalize);
		}

		let mut target_id = lab_id.clone();

		if !test_result_success.clone() {
			target_id = requester_id.clone();
		}

		match CurrencyOf::<T>::withdraw(
			&Self::staking_account_id(request_id.clone()),
			service_invoice.pay_amount.clone(),
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(&target_id, imb);
			},
			_ => {
				return Err(Error::<T>::BadSignature)
			},
		}

		request.status = RequestStatus::Finalized;
		RequestById::<T>::insert(request_id.clone(), request.clone());

		Ok(service_invoice.clone())
	}
}
