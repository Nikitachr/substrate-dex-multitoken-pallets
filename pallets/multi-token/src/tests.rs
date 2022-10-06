use frame_support::{ assert_ok, assert_noop};
use frame_benchmarking::{account, whitelisted_caller};
use crate::{mock::*, Error};
use frame_system;

type AccountId = <Test as frame_system::Config>::AccountId;

#[test]
fn should_set_approval() {
	new_test_ext().execute_with(|| {
        let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::set_approval(Origin::signed(whitelisted_caller()), recipient.clone(), true));
		assert_eq!(MultiToken::approvals(whitelisted_caller::<AccountId>(), recipient), Some(true));
	});
}

#[test]
fn should_mint() {
	new_test_ext().execute_with(|| {
		assert_ok!(MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 10));
		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(10));
	});
}

#[test]
fn should_mint_batch() {
	new_test_ext().execute_with(|| {
		assert_ok!(MultiToken::mint_batch(Origin::signed(whitelisted_caller()), [0, 1].to_vec(), [2, 10].to_vec()));
		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(2));
		assert_eq!(MultiToken::balances(1, whitelisted_caller::<AccountId>()), Some(10));
	});
}

#[test]
fn should_revet_mint_batch_with_wrong_args() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MultiToken::mint_batch(Origin::signed(whitelisted_caller()), [0, 1].to_vec(), [2].to_vec()),
			Error::<Test>::ShouldBeSameLength
		);
	});
}

#[test]
fn should_transfer() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);

		assert_ok!(MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 10));
		assert_ok!(MultiToken::transfer_to(Origin::signed(whitelisted_caller()), recipient.clone(), 0, 5));

		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(5));
		assert_eq!(MultiToken::balances(0, recipient), Some(5));
	});	
}

#[test]
fn should_revert_transfer_if_not_enough_value() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 3));
		assert_noop!(
			MultiToken::transfer_to(Origin::signed(whitelisted_caller()), recipient.clone(), 0, 5),
			Error::<Test>::NotEnoughOwned
		);
	});	
}

#[test]
fn should_transfer_batch() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);

		assert_ok!(MultiToken::mint_batch(Origin::signed(whitelisted_caller()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_ok!(MultiToken::transfer_batch_to(Origin::signed(whitelisted_caller()), recipient.clone(), [0, 1].to_vec(), [2, 3].to_vec()));
		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(3));
		assert_eq!(MultiToken::balances(1, whitelisted_caller::<AccountId>()), Some(7));
		assert_eq!(MultiToken::balances(0, recipient), Some(2));
		assert_eq!(MultiToken::balances(1, recipient), Some(3));

	});	
}

#[test]
fn should_revert_transfer_batch_if_not_enough_value() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);

		assert_ok!(MultiToken::mint_batch(Origin::signed(whitelisted_caller()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_noop!(
			MultiToken::transfer_batch_to(Origin::signed(whitelisted_caller()), recipient.clone(), [0, 1].to_vec(), [2, 20].to_vec()),
			Error::<Test>::NotEnoughOwned
		);
	});	
}

#[test]
fn should_transfer_from() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint(Origin::signed(recipient.clone()), 0, 10));
		assert_ok!(MultiToken::set_approval(Origin::signed(recipient.clone()), whitelisted_caller(), true));
		assert_ok!(MultiToken::transfer_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), 0, 5));
		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(5));
		assert_eq!(MultiToken::balances(0, recipient), Some(5));
	});	
}

#[test]
fn should_revert_transfer_from_if_not_approved() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint(Origin::signed(recipient.clone()), 0, 10));
		assert_noop!(
			MultiToken::transfer_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), 0, 5),
			Error::<Test>::NotApproved
		);
	});	
}


#[test]
fn should_revert_transfer_from_if_not_enough_value() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint(Origin::signed(recipient.clone()), 0, 10));
		assert_ok!(MultiToken::set_approval(Origin::signed(recipient.clone()), whitelisted_caller(), true));
		assert_noop!(
			MultiToken::transfer_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), 0, 20),
			Error::<Test>::NotEnoughOwned
		);
	});	
}

#[test]
fn should_transfer_batch_from() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint_batch(Origin::signed(recipient.clone()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_ok!(MultiToken::set_approval(Origin::signed(recipient.clone()), whitelisted_caller(), true));
		assert_ok!(MultiToken::transfer_batch_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_eq!(MultiToken::balances(0, whitelisted_caller::<AccountId>()), Some(5));
		assert_eq!(MultiToken::balances(1, whitelisted_caller::<AccountId>()), Some(10));
		assert_eq!(MultiToken::balances(0, recipient), Some(0));
		assert_eq!(MultiToken::balances(1, recipient), Some(0));
	});	
}

#[test]
fn should_revert_transfer_batch_from_if_wrong_args() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint_batch(Origin::signed(recipient.clone()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_ok!(MultiToken::set_approval(Origin::signed(recipient.clone()), whitelisted_caller(), true));
		assert_noop!(
			MultiToken::transfer_batch_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), [0, 1].to_vec(), [5].to_vec()),
			Error::<Test>::ShouldBeSameLength
		);
	});	
}

#[test]
fn should_revert_transfer_batch_from_if_not_approved() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint_batch(Origin::signed(recipient.clone()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_noop!(
			MultiToken::transfer_batch_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), [0, 1].to_vec(), [5, 10].to_vec()),
			Error::<Test>::NotApproved
		);
	});	
}

#[test]
fn should_revert_transfer_batch_from_if_not_enough_value() {
	new_test_ext().execute_with(|| {
		let recipient: AccountId = account("recipient", 0, 1);
		assert_ok!(MultiToken::mint_batch(Origin::signed(recipient.clone()), [0, 1].to_vec(), [5, 10].to_vec()));
		assert_ok!(MultiToken::set_approval(Origin::signed(recipient.clone()), whitelisted_caller(), true));
		assert_noop!(
			MultiToken::transfer_batch_from(Origin::signed(whitelisted_caller()), recipient.clone(), whitelisted_caller(), [0, 1].to_vec(), [5, 15].to_vec()),
			Error::<Test>::NotEnoughOwned
		);
	});	
}
