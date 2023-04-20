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
	use frame_support::pallet_prelude::{*, DispatchResult};
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
	pub struct Company {
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
	pub type AccountToCompany<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Company, ValueQuery>;

	///Contract data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct SupplyContract {
		pub id: u64,
		pub seller_id: u64,
		pub buyer_id: u64,
		pub products: Vec<u64>,
		pub delivered: bool,
		pub iou: u64,
		pub contract_value: u64,
		pub contract_fulfilled: bool,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn supply_contract_info)]
	pub type AccountToSupplyContract<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, SupplyContract, ValueQuery>;

	///product data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Product {
		pub id: u64,
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub owner: u64,
		pub previous_owners: Vec<u64>,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn product_info)]
	pub type AccountToProduct<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Product, ValueQuery>;

	///IOU data
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct IOU {
		pub id: u64,
		pub debtor: u64,
		pub creditor: u64,
		pub amount: u64,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn iou_info)]
	pub type AccountToIOU<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, IOU, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CompanyCreated { company: T::AccountId },
		SupplyContractCreated { contract: T::AccountId },
		ProductCreated { product: T::AccountId },
		IOUCreated { iou: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		CompanynameTooLong,
		AboutMeTooLong,
		IdTooSmall,
		IdTooBig,
		ProductIdNotFound,
		NotProductOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable calls go here!
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_company(
			origin: OriginFor<T>,
			name: Vec<u8>,
			id: u64,
			about_me: Vec<u64>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(name.len() <= 64, Error::<T>::CompanynameTooLong);
			ensure!(about_me.len() <= 2000, Error::<T>::AboutMeTooLong);
			ensure!(id > 0, Error::<T>::IdTooSmall);
			ensure!(id < 10000000000000, Error::<T>::IdTooBig);

			let new_company = Company { name, id, about_me };

			<AccountToCompany<T>>::insert(&sender, new_company);
			Self::deposit_event(Event::CompanyCreated { company: sender });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_product(
			origin: OriginFor<T>,
			id: u64,
			name: Vec<u8>,
			description: Vec<u8>,
			owner: u64,
			previous_owners: Vec<u64>,
		)-> DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(id > 0, Error::<T>::IdTooSmall);
			ensure!(id < 10000000000000, Error::<T>::IdTooBig);

			let new_product = Product { id, name, description, owner, previous_owners };

			<AccountToProduct<T>>::insert(&sender, new_product);
			Self::deposit_event(Event::ProductCreated { product: sender });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_supply_contract(
			origin: OriginFor<T>,
			id: u64,
			seller_id: u64,
			buyer_id: u64,
			products: Vec<u64>,
			delivered: bool,
			iou: u64,
			contract_value: u64,
			contract_fulfilled: bool,
		)-> DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(id > 0, Error::<T>::IdTooSmall);
			ensure!(id < 10000000000000, Error::<T>::IdTooBig);

			let new_supply_contract = SupplyContract { id, seller_id, buyer_id, products, delivered, iou, contract_value, contract_fulfilled };

			<AccountToSupplyContract<T>>::insert(&sender, new_supply_contract);
			Self::deposit_event(Event::SupplyContractCreated { contract: sender });
			Ok(())
		}

		// 		pub id: u64,
		// pub debtor: u64,
		// pub creditor: u64,
		// pub amount: u64,
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_iou(
			origin: OriginFor<T>,
			id: u64,
			debtor: u64,
			creditor: u64,
			amount: u64,
		)-> DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(id > 0, Error::<T>::IdTooSmall);
			ensure!(id < 10000000000000, Error::<T>::IdTooBig);

			let new_iou = IOU { id, debtor, creditor, amount };

			<AccountToIOU<T>>::insert(&sender, new_iou);
			Self::deposit_event(Event::IOUCreated { iou: sender });
			Ok(())
		}

	}
}
