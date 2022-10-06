#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use crate::multitoken::MultiToken;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::*;

use frame_support::pallet_prelude::DispatchResult;
// use frame_system::pallet_prelude::OriginFor;
use sp_std::vec::Vec;
pub mod multitoken;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

impl<T:Config> MultiToken<T::AccountId> for Pallet<T> {
	fn balances(id: &u64, account: &T::AccountId) -> u64 {
		match Self::balances(id, account) {
			Some(x) => x,
			None => 0
		}
	}

	fn transfer_to(_origin: &T::AccountId, _to: &T::AccountId, _id: &u64, _amount: &u64) -> DispatchResult {
		ensure!(Self::balances(_id, _origin) >= Some(*_amount), Error::<T>::NotEnoughOwned);
		Balances::<T>::try_mutate(_id, _origin, |balance| {
			match balance {
				Some(x) => {
					*x -= _amount;
					Ok(())
				},
				None => Err(())
			}
		});

		match Balances::<T>::get(_id, _to){
			Some(x) => Balances::<T>::insert(_id, _to, x + _amount),
			None => Balances::<T>::insert(_id, _to, _amount),
		};
		Ok(())
	}

	fn mint(_origin: &T::AccountId, _id: &u64, _amount: &u64) -> DispatchResult {
		match Balances::<T>::get(_id, _origin){
			Some(x) => Balances::<T>::insert(_id, _origin, x + _amount),
			None => Balances::<T>::insert(_id, _origin, _amount),
		};
		Ok(())
	}
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use crate::weights::WeightInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MintSingle { account: T::AccountId, id: u64, value: u64 },
		MintBatch { account: T::AccountId, id: Vec<u64>, value: Vec<u64> },
		TransferSingle { operator: T::AccountId, from: T::AccountId, to: T::AccountId, id: u64, value: u64 },
		TransferBatch { operator: T::AccountId, from: T::AccountId, to: T::AccountId, id: Vec<u64>, value: Vec<u64> },
		ApprovalForAll { account: T::AccountId, operator: T::AccountId, approved: bool }
	}

	#[pallet::error]
	pub enum Error<T> {
		ShouldBeSameLength,
		NotEnoughOwned,
		NotApproved
	}

	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub type Balances<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u64, Blake2_128Concat, T::AccountId, u64>;

	
	#[pallet::storage]
	#[pallet::getter(fn approvals)]
	pub type Approvals<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, bool>;

	#[pallet:: call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::set_approval())]
		pub fn set_approval(origin: OriginFor<T>, account: T::AccountId, is_approved: bool) -> DispatchResult {
			
			let sender = &ensure_signed(origin)?;

			Approvals::<T>::insert(sender, &account, &is_approved);
			Self::deposit_event(Event::<T>::ApprovalForAll { account: sender.clone(), operator: account, approved: is_approved });
			
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::mint())]
		pub fn mint(origin: OriginFor<T>, _id: u64, _amount: u64) -> DispatchResult {
			
			let sender = &ensure_signed(origin)?;

			match Balances::<T>::get(_id, sender){
				Some(x) => Balances::<T>::insert(&_id, sender, x + &_amount),
				None => Balances::<T>::insert(&_id, sender, &_amount),
			};

			Self::deposit_event(Event::<T>::MintSingle { account: sender.clone(), id: _id, value: _amount });

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::mint_batch(_id.len() as u64))]
		pub fn mint_batch(origin: OriginFor<T>, _id: Vec<u64>, _amount: Vec<u64>) -> DispatchResult {
	
			let sender = &ensure_signed(origin)?;
			ensure!(_id.len() == _amount.len(), Error::<T>::ShouldBeSameLength);

			for i in 0.._id.len() {
				match Balances::<T>::get(_id[i], sender){
					Some(x) => Balances::<T>::insert(_id[i], sender, x + _amount[i]),
					None => Balances::<T>::insert(_id[i], sender, _amount[i]),
				};
			}

			Self::deposit_event(Event::<T>::MintBatch { account: sender.clone(), id: _id, value: _amount });
			
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer_to())]
		pub fn transfer_to(origin: OriginFor<T>, _to: T::AccountId, _id: u64, _amount: u64) -> DispatchResult {

			let sender = &ensure_signed(origin)?;
			ensure!(Balances::<T>::get(&_id, sender) >= Some(_amount), Error::<T>::NotEnoughOwned);

			Balances::<T>::try_mutate(&_id, sender, |balance| {
				match balance {
					Some(x) => {
						*x -= &_amount;
						Ok(())
					},
					None => Err(())
				}
			});

			match Balances::<T>::get(&_id, &_to){
				Some(x) => Balances::<T>::insert(&_id, &_to, x + &_amount),
				None => Balances::<T>::insert(&_id, &_to, &_amount),
			};

			Self::deposit_event(Event::<T>::TransferSingle {
				operator: sender.clone(),
				from: sender.clone(),
				to: _to,
				id: _id,
				value: _amount
			});
			
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer_batch_to(_id.len() as u64))]
		pub fn transfer_batch_to(origin: OriginFor<T>, _to: T::AccountId, _id: Vec<u64>, _amount: Vec<u64>) -> DispatchResult {

			let sender = &ensure_signed(origin)?;
			ensure!(_id.len() == _amount.len(), Error::<T>::ShouldBeSameLength);

			for it in _id.iter().zip(_amount.iter()) {
				let (&id, &amount) = it;
				ensure!(Balances::<T>::get(&id, &sender) >= Some(amount), Error::<T>::NotEnoughOwned);

				Balances::<T>::try_mutate(&id, &sender, |balance| {
					match balance {
						Some(x) => {
							*x -= &amount;
							Ok(())
						},
						None => Err(())
					}
				});

				match Balances::<T>::get(&id, &_to){
					Some(x) => Balances::<T>::insert(&id, &_to, x + &amount),
					None => Balances::<T>::insert(&id, &_to, &amount),
				};
			}
			
			Self::deposit_event(Event::<T>::TransferBatch {
				operator: sender.clone(),
				from: sender.clone(),
				to: _to,
				id: _id,
				value: _amount
			});

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer_from())]
		pub fn transfer_from(origin: OriginFor<T>, _from:T::AccountId, _to: T::AccountId, _id: u64, _amount: u64) -> DispatchResult {

			let sender = &ensure_signed(origin)?;

			ensure!(Balances::<T>::get(&_id, &_from) >= Some(_amount), Error::<T>::NotEnoughOwned);
			ensure!(Approvals::<T>::get(&_from, &_to) == Some(true), Error::<T>::NotApproved);

			Balances::<T>::try_mutate(&_id, &_from, |balance| {
				match balance {
					Some(x) => {
						*x -= _amount;
						Ok(())
					},
					None => Err(())
				}
			});

			match Balances::<T>::get(&_id, &_to){
				Some(x) => Balances::<T>::insert(&_id,& _to, x + _amount),
				None => Balances::<T>::insert(&_id, &_to, _amount),
			};

			Self::deposit_event(Event::<T>::TransferSingle {
				operator: sender.clone(),
				from: _from,
				to: _to,
				id: _id,
				value: _amount
			});
			
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer_batch_from(_id.len() as u64))]
		pub fn transfer_batch_from(origin: OriginFor<T>, _from: T::AccountId, _to: T::AccountId, _id: Vec<u64>, _amount: Vec<u64>) -> DispatchResult {

			let sender = &ensure_signed(origin)?;
			ensure!(_id.len() == _amount.len(), Error::<T>::ShouldBeSameLength);
			ensure!(Approvals::<T>::get(&_from, &_to) == Some(true), Error::<T>::NotApproved);

			for it in _id.iter().zip(_amount.iter()) {
				let (&id, &amount) = it;
				ensure!(Balances::<T>::get(id, &_from) >= Some(amount), Error::<T>::NotEnoughOwned);

				Balances::<T>::try_mutate(id, &_from, |balance| {
					match balance {
						Some(x) => {
							*x -= amount;
							Ok(())
						},
						None => Err(())
					}
				});

				match Balances::<T>::get(id, &_to){
					Some(x) => Balances::<T>::insert(id, &_to, x + amount),
					None => Balances::<T>::insert(id, &_to, amount),
				};
			}

			Self::deposit_event(Event::<T>::TransferBatch {
				operator: sender.clone(),
				from: _from,
				to: _to,
				id: _id,
				value: _amount
			});
			
			Ok(())
		}
	
	}
}