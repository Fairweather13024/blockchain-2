#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	///Company data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Company{
		///number id of the company
		pub id: u64,
		///company name stored as an array of bytes
		pub name: Vec<u8>,
		///companys about information
		pub about_me: Vec<u64>,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn company_info)]
	pub type AccountToCompany<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Company, ValueQuery>;


	///Contract data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct SupplyContract{
		///number id of the company
		pub id: u64,
		pub sellerId: u64,
		pub buyerId: u64,
		///companys about information
		pub products: Vec<u64>,
		pub delivered : bool,
		pub IOU : u64,
		pub contract_value : u64,
		pub contract_fulfilled : bool,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn supply_contract_info)]
	pub type AccountToSupplyContract<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, SupplyContract, ValueQuery>;


	///product data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Product{
		pub id: u64,
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub owner: u64,
		pub previous_owners: Vec<u64>,
		pub timestamp: u64,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn product_info)]
	pub type AccountToProduct<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Product, ValueQuery>;


	///IOU data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct IOU{
		pub id: u64,
		pub debtor: u64,
		pub creditor: u64,
		pub amount: u64,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn iou_info)]
	pub type AccountToIOU<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, IOU, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable calls go here!
	}
}
