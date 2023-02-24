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

	///Implement custom struct
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, TypeInfo)]
	pub struct UserInfo{
		///user name stored as an array of bytes
		pub username: Vec<u8>,
		///number id of the user
		pub id: i64,
		///users about information
		pub about_me: Vec<u8>,
	}

	///storage map to interact with the node's storage
	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountToUserInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;


	///custom struct for a product in a supply chain
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, TypeInfo)]
	pub struct Product {
		pub id: u64,
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub owner: T::AccountId,
		pub previous_owners: Vec<T::AccountId>,
		pub timestamp: T::Moment,
	}

	#[pallet::storage]
	#[pallet::getter(fn products)]
	pub type Products<T: Config> = StorageMap<_, Blake2_128Concat, u64, Product, OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserCreated {user: T::AccountId},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The username is too long
		UsernameTooLong,
		/// The about me is too long
		AboutMeTooLong,
		/// The id is too small
		IdTooSmall,
		/// The id is too big
		IdTooBig,
		/// The product id already exists
		ProductIdExists,
		/// The product id does not exist
		ProductIdNotFound,
		/// The user is not the owner of the product
		NotProductOwner,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable calls go here!
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_user(origin: OriginFor<T>, username: Vec<u8>, id: i64, about_me: Vec<u8>) -> DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(username.len() <= 32, Error::<T>::UsernameTooLong);
			ensure!(about_me.len() <= 256, Error::<T>::AboutMeTooLong);
			ensure!(id > 0, Error::<T>::IdTooSmall);
			ensure!(id < 100000000, Error::<T>::IdTooBig);

			let new_user = UserInfo{
				username,
				id,
				about_me,
			};

			<AccountToUserInfo<T>>::insert(&sender, new_user);
			Self::deposit_event(Event::UserCreated{user: sender});
			Ok(())
	}
	pub fn create_product(
        origin: OriginFor<T>,
        id: u64,
        name: Vec<u8>,
        description: Vec<u8>,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        ensure!(!Products::<T>::contains_key(id), Error::<T>::ProductIdExists);

        let product = Product {
            id,
            name,
            description,
            owner: sender.clone(),
            previous_owners: vec![],
            timestamp: <frame_system::Pallet<T>>::block_number(),
        };

        Products::<T>::insert(id, product);
        Self::deposit_event(Event::ProductCreated(sender, id));
        Ok(())
    }

    pub fn transfer_product(
        origin: OriginFor<T>,
        id: u64,
        to: T::AccountId,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        let mut product = Products::<T>::get(id).ok_or(Error::<T>::ProductIdNotFound)?;
        ensure!(product.owner == sender, Error::<T>::NotProductOwner);

        product.previous_owners.push(sender.clone());
        product.owner = to.clone();

        Products::<T>::insert(id, product);
        Self::deposit_event(Event::ProductTransferred(sender, to, id));
        Ok(())
    }

    pub fn get_product(id: u64) -> Option<Product> {
        Products::<T>::get(id)
    }
}
}