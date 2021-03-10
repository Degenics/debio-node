#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, Randomness, // Currency, ExistenceRequirement,
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{
    RuntimeDebug,
    //traits::Hash
};
use frame_support::sp_std::prelude::*;
use frame_support::sp_std::convert::{TryInto, TryFrom};
use escrow_controller::EscrowController;


pub trait Trait: frame_system::Trait
    + services::Trait
    + escrow::Trait
    + specimen::Trait
    + pallet_timestamp::Trait
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type RandomnessSource: Randomness<Self::Hash>;
    // type Hashing: Hash<Output = Self::Hash>;
}

type MomentOf<T> = <T as pallet_timestamp::Trait>::Moment;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum OrderStatus {
    Unpaid,
    Paid,
    Fulfilled,
    Refunded,
}
impl Default for OrderStatus {
    fn default() -> Self { OrderStatus::Unpaid }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Order<Hash, AccountId, Moment> {
    id: Hash,
    service_id: Hash,
    customer_id: AccountId,
    lab_id: AccountId,
    escrow_id: AccountId,
    created_at: Moment,
    updated_at: Moment,
    status: OrderStatus,
}
impl<Hash, AccountId, Moment> Order<Hash, AccountId, Moment> {
    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_created_at(&self) -> &Moment {
        &self.created_at
    }

    pub fn get_service_id(&self) -> &Hash {
        &self.service_id
    }
}


type OrderIds<T> = Vec<<T as frame_system::Trait>::Hash>;

decl_storage! {
    trait Store for Module<T: Trait> as OrdersStorage {
        pub Orders get(fn order_by_id): map hasher(blake2_128_concat)
                T::Hash => Option<Order<T::Hash, T::AccountId, T::Moment>>;

        // List of order ids by customer
        pub CustomerOrders get(fn orders_by_costumer_id): map hasher(blake2_128_concat)
                T::AccountId => Option<OrderIds<T>>;

        // List of order ids by lab
        pub LabOrders get(fn orders_by_lab_id): map hasher(blake2_128_concat)
                T::AccountId => Option<OrderIds<T>>;
    }
}

decl_event!(
    pub enum Event<T> where
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as frame_system::Trait>::Hash,
        Moment = <T as pallet_timestamp::Trait>::Moment,
    {
        /// Order created
        /// parameters, [Order]
        OrderCreated(Order<Hash, AccountId, Moment>),
        /// Order paid
        /// parameters, [Order, customer, lab]
        OrderPaid(Order<Hash, AccountId, Moment>),
        /// Order Fulfilled
        /// parameters, [Order, customer, lab]
        OrderFulfilled(Order<Hash, AccountId, Moment>),
        /// Order Refunded
        /// parameters, [Order, customer, lab]
        OrderRefunded(Order<Hash, AccountId, Moment>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Lab id does not exist
        LabDoesNotExist,
        /// Service id does not exist
        ServiceDoesNotExist,
        /// Order does not exist
        OrderNotFound,
        /// Escrow not found
        EscrowNotFound,
        /// Unauthorized to fulfill order - user is not the lab who owns the service
        UnauthorizedOrderFulfillment,
        /// Can not fulfill order before Specimen is processed
        SpecimenNotProcessed,
        /// Refund not allowed, Order is not expired yet
        OrderNotYetExpired,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn create_order(origin, service_id: T::Hash) -> dispatch::DispatchResult {
            let customer_id = ensure_signed(origin)?;
            let service = services::Module::<T>::service_by_id(service_id);
            match service {
                None => Err(Error::<T>::ServiceDoesNotExist)?,
                Some(service) => {
                    let order_id = Self::generate_hash(&customer_id);
                    let service_id = service.get_id();
                    let lab_id = service.get_lab_id();
                    let created_at = pallet_timestamp::Module::<T>::get();

                    // Create escrow
                    let escrow_account_id = escrow::Module::<T>::create_escrow(
                        &order_id,
                        &customer_id, // buyer id
                        &lab_id, // seller id
                        service.get_price(), // amount to pay
                        &created_at,
                    );

                    // Create specimen
                    let _specimen = specimen::Module::<T>::create_specimen(
                        &order_id,
                        &service_id,
                        &customer_id, // specimen owner id
                        &lab_id,
                        &created_at,
                    );

                    // Create order
                    let order = Order {
                        id: order_id,
                        customer_id: customer_id.clone(),
                        service_id: *service_id,
                        lab_id: lab_id.clone(),
                        escrow_id: escrow_account_id.clone(),
                        created_at: created_at,
                        updated_at: created_at,
                        status: OrderStatus::Unpaid,
                    };
                    Orders::<T>::insert(&order_id, &order);

                    // Store order id reference for Customers
                    let orders = CustomerOrders::<T>::get(&customer_id);
                    if orders == None {
                        let mut orders = Vec::<T::Hash>::new();
                        orders.push(order_id);
                        CustomerOrders::<T>::insert(&customer_id, orders);
                    } else {
                        let mut orders = orders.unwrap();
                        orders.push(order_id);
                        CustomerOrders::<T>::insert(&customer_id, orders);
                    }
                    let orders = CustomerOrders::<T>::get(&customer_id);
                    debug::info!("** ---- CustomerOrders ---- **: {:?}", orders);


                    // Store order id reference for Labs
                    let orders = LabOrders::<T>::get(&lab_id);
                    if orders == None {
                        let mut orders = Vec::<T::Hash>::new();
                        orders.push(order_id);
                        LabOrders::<T>::insert(&lab_id, orders);
                    } else {
                        let mut orders = orders.unwrap();
                        orders.push(order_id);
                        LabOrders::<T>::insert(&lab_id, orders);
                    }
                    let orders = LabOrders::<T>::get(&customer_id);
                    debug::info!("** ---- LabOrders ---- **: {:?}", orders);


                    Self::deposit_event(RawEvent::OrderCreated(order));
                    Ok(())
                }
            }
        }


        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pay_order(origin, order_id: T::Hash) -> dispatch::DispatchResult {
            let customer_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }

            // Pay to escrow
            let _escrow = escrow::Module::<T>::deposit(&order_id, &customer_id);

            // Set order status to paid
            let order = Self::update_order_status(&order_id, OrderStatus::Paid);
            let order = order.unwrap();

            Self::deposit_event(RawEvent::OrderPaid(order.clone()));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn fulfill_order(origin, order_id: T::Hash) -> dispatch::DispatchResult {
            let user_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }
            let order = order.unwrap();

            // Only the lab who owns the service in the order can fulfill
            let is_lab = user_id == order.lab_id;
            if !is_lab {
                return Err(Error::<T>::UnauthorizedOrderFulfillment)?;
            }

            // Specimen has to be processed before order is fulfilled
            let is_specimen_processed = specimen::Module::<T>::is_status(
                &order_id,
                specimen::SpecimenStatus::Processed
            );
            if !is_specimen_processed {
                return Err(Error::<T>::SpecimenNotProcessed)?;
            }

            let order = Self::update_order_status(&order.id, OrderStatus::Fulfilled);
            let order = order.unwrap();

            // Release funds to lab
            escrow::Module::<T>::release(&order.id);

            Self::deposit_event(RawEvent::OrderFulfilled(order.clone()));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn refund_order(origin, order_id: T::Hash) -> dispatch::DispatchResult {
            let _user_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }
            let order = order.unwrap();

            // Check if order expired ------------------
            let now = pallet_timestamp::Module::<T>::get();
            let order_created_at = order.created_at.clone();
            // convert to u64
            let order_created_at_ms = TryInto::<u64>::try_into(order_created_at).ok().unwrap();
            // Add 7 days
            let seven_days_ms = u64::try_from(chrono::Duration::days(7).num_milliseconds()).ok().unwrap();
            let expires_at_ms = order_created_at_ms + seven_days_ms;
            // convert back to Moment
            let expires_at = TryInto::<MomentOf<T>>::try_into(expires_at_ms).ok().unwrap();


            // Check if specimen rejected
            let is_specimen_rejected = specimen::Module::<T>::is_status(
                &order_id,
                specimen::SpecimenStatus::Rejected
            );

            // Can refund if order expired or specimen rejected
            let can_refund = now > expires_at || is_specimen_rejected;
            if !can_refund {
                return Err(Error::<T>::OrderNotYetExpired)?;
            }

            // Refund back to customer
            escrow::Module::<T>::refund(&order_id);

            let order = Self::update_order_status(&order_id, OrderStatus::Refunded);
            let order = order.unwrap();

            Self::deposit_event(RawEvent::OrderRefunded(order.clone()));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // TODO: Maybe extract this fn as a separate module (this is used by pallet services also)
    fn generate_hash(account_id: &T::AccountId) -> T::Hash
    {
        let account_info = frame_system::Module::<T>::account(account_id);
        let hash = <T as Trait>::RandomnessSource::random(&account_info.nonce.encode());
        // let hash = <T as Trait>::Hashing::hash(random_result);
        return hash;
    }

    fn update_order_status(order_id: &T::Hash, status: OrderStatus)
        -> Option<Order<T::Hash, T::AccountId, T::Moment>>
    {
        Orders::<T>::mutate(order_id, |order| {
            match order {
                None => None,
                Some(order) => {
                    order.status = status;
                    order.updated_at = pallet_timestamp::Module::<T>::get();
                    Some(order.clone())
                }
            }
        })
    }
}


// TODO: Is it possible to trigger this from escrow pallet
// when the escrow account is paid by straight transfer not
// dispatchable calls??
impl<T: Trait> EscrowController<T> for Module<T> {
    fn on_escrow_paid(controller_id: &T::Hash) -> () {
        let order_id = controller_id;
        Orders::<T>::mutate(order_id, |order| {
            match order {
                None => (),
                Some(order) => {
                    order.status = OrderStatus::Paid;
                }
            }
        })
    }
}