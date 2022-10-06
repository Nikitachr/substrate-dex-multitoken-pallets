#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

// pub use pallet_multi_token;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use pallet_multi_token::multitoken::MultiToken;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MultiToken: MultiToken<Self::AccountId>;
		#[pallet::constant]
        type Fee: Get<u64>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
		WrongRatio,
		NoLiquidity,
	}

	#[pallet::storage]
	pub type Pool<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub type FirstTokeId<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub type SecondTokeId<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub type LPTokenTotalSupply<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type BalanceOf<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet:: call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn init(
			origin: OriginFor<T>,
			pool_address: T::AccountId,
			first_token_id: u64,
			first_token_amount: u64,
			second_token_id: u64,
			second_token_amount: u64
		) -> DispatchResult {
			let sender = &ensure_signed(origin)?;

			T::MultiToken::transfer_to(sender, &pool_address, &first_token_id, &first_token_amount)?;
			T::MultiToken::transfer_to(sender, &pool_address, &second_token_id, &second_token_amount)?;
			BalanceOf::<T>::set(sender, first_token_amount);
			LPTokenTotalSupply::<T>::set(first_token_amount);
			Pool::<T>::set(Some(pool_address));
			FirstTokeId::<T>::set(first_token_id);
			SecondTokeId::<T>::set(second_token_id);

			Ok(())
		}
		
		#[pallet::weight(1000)]
		pub fn swap_token(
			origin: OriginFor<T>,
			_token_id: u64,
			_amount: u64
		) -> DispatchResult {
			let sender = &ensure_signed(origin)?;
			let pool_address: T::AccountId = Pool::<T>::get().expect("Pool is not initialized");

			if FirstTokeId::<T>::get() == _token_id {
					let mut received_value = (T::MultiToken::balances(&SecondTokeId::<T>::get(), &pool_address) * &_amount)
					/ (T::MultiToken::balances(&FirstTokeId::<T>::get(), &pool_address) + &_amount);
					received_value = received_value * (100 - T::Fee::get()) / 100;
					T::MultiToken::transfer_to(sender, &pool_address, &_token_id, &_amount)?;
					T::MultiToken::transfer_to(&pool_address, sender, &SecondTokeId::<T>::get(), &received_value)?;
				} else {
					let mut received_value = (T::MultiToken::balances(&FirstTokeId::<T>::get(), &pool_address) * &_amount)
					/ (T::MultiToken::balances(&SecondTokeId::<T>::get(), &pool_address) + &_amount);
					received_value = received_value * (100 - T::Fee::get()) / 100;
					T::MultiToken::transfer_to(sender, &pool_address, &_token_id, &_amount)?;
					T::MultiToken::transfer_to(&pool_address, sender, &FirstTokeId::<T>::get(), &received_value)?;
				}
			
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn deposit(
			origin: OriginFor<T>,
			_first_token_amount: u64,
			_second_token_amount: u64
		) -> DispatchResult {
			let sender = &ensure_signed(origin)?;
			let pool_address: &T::AccountId = &Pool::<T>::get().expect("Pool is not initialized");
			let first_token_balance = &T::MultiToken::balances(&FirstTokeId::<T>::get(), &pool_address);
			let second_token_balance = &T::MultiToken::balances(&SecondTokeId::<T>::get(), &pool_address);
			let liquidity = _first_token_amount * LPTokenTotalSupply::<T>::get() / first_token_balance;
			let required_second_token_amount = second_token_balance * _first_token_amount / first_token_balance;
			ensure!(_second_token_amount >= required_second_token_amount, Error::<T>::WrongRatio);
			T::MultiToken::transfer_to(sender, pool_address, &FirstTokeId::<T>::get(), &_first_token_amount)?;
			T::MultiToken::transfer_to(sender, pool_address, &SecondTokeId::<T>::get(), &required_second_token_amount)?;
			LPTokenTotalSupply::<T>::set(LPTokenTotalSupply::<T>::get() + &liquidity);
			BalanceOf::<T>::mutate(sender, |x| *x += liquidity);
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn withdraw(
			origin: OriginFor<T>
		) -> DispatchResult {
			let sender = &ensure_signed(origin)?;
			let pool_address: &T::AccountId = &Pool::<T>::get().expect("Pool is not initialized");
			let lp_balance = BalanceOf::<T>::get(sender);
			ensure!(lp_balance >= 0, Error::<T>::NoLiquidity);
			let first_token_amount = T::MultiToken::balances(&FirstTokeId::<T>::get(), pool_address) * lp_balance / LPTokenTotalSupply::<T>::get();
			let second_token_amount = T::MultiToken::balances(&SecondTokeId::<T>::get(), pool_address) * lp_balance / LPTokenTotalSupply::<T>::get();

			LPTokenTotalSupply::<T>::set(LPTokenTotalSupply::<T>::get() - lp_balance);
			BalanceOf::<T>::set(sender, 0);

			T::MultiToken::transfer_to(pool_address, sender, &FirstTokeId::<T>::get(), &first_token_amount)?;
			T::MultiToken::transfer_to(pool_address, sender, &SecondTokeId::<T>::get(), &second_token_amount)?;

			Ok(())
		}
	}

	

}