use frame_support::pallet_prelude::DispatchResult;

pub trait MultiToken<AccountId> {
	fn balances(id: &u64, account: &AccountId) -> u64;
	fn transfer_to(_origin: &AccountId, _to: &AccountId, _id: &u64, _amount: &u64) -> DispatchResult;
	fn mint(_origin: &AccountId, _id: &u64, _amount: &u64) -> DispatchResult;
}