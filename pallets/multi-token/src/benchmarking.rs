use super::*;

#[allow(unused)]
use crate::Pallet as MultiToken;
use frame_benchmarking::{account, benchmarks, whitelisted_caller, vec};
use frame_system::RawOrigin;
use frame_support::{
	ensure,
};

benchmarks! {
	set_approval {
		let recipient: T::AccountId = account("recipient", 0, 1);
	}: _(RawOrigin::Signed(whitelisted_caller()), recipient, true)

	mint {
	}: _(RawOrigin::Signed(whitelisted_caller()), 10, 10)

	mint_batch {
		let l in 0 .. 10 as u64;
        let data = vec![1; l as usize];
	}: _(RawOrigin::Signed(whitelisted_caller()), data.clone(), data.clone())
	verify {
        ensure!(data.len() == data.len(), "arrays should be the same length");
	}

	transfer_to {
		let recipient: T::AccountId = account("recipient", 0, 1);
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(whitelisted_caller()));
		MultiToken::<T>::mint(caller_origin, 0, 10);
	}: _(RawOrigin::Signed(whitelisted_caller()), recipient, 0, 10)

	transfer_batch_to {
		let recipient: T::AccountId = account("recipient", 0, 1);
		let l in 0 .. 10 as u64;
        let data = vec![1; l as usize];
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(whitelisted_caller()));
		MultiToken::<T>::mint_batch(caller_origin, data.clone(), data.clone());
	}: _(RawOrigin::Signed(whitelisted_caller()), recipient, data.clone(), data.clone())
	verify {
        ensure!(data.len() == data.len(), "arrays should be the same length");
	}

	transfer_from {
		let recipient: T::AccountId = account("recipient", 0, 1);
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(recipient.clone()));
		MultiToken::<T>::mint(caller_origin.clone(), 0, 10);
		MultiToken::<T>::set_approval(caller_origin, whitelisted_caller(), true);
	}: _(RawOrigin::Signed(whitelisted_caller()), recipient, whitelisted_caller(), 0, 10)

	transfer_batch_from {
		let recipient: T::AccountId = account("recipient", 0, 1);
		let l in 0 .. 10 as u64;
        let data = vec![1; l as usize];
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(recipient.clone()));
		MultiToken::<T>::mint_batch(caller_origin.clone(), data.clone(), data.clone());
		MultiToken::<T>::set_approval(caller_origin, whitelisted_caller(), true);
	}: _(RawOrigin::Signed(whitelisted_caller()), recipient, whitelisted_caller(), data.clone(), data.clone())
	verify {
        ensure!(data.len() == data.len(), "arrays should be the same length");
	}

}

