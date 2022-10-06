use frame_support::{ assert_ok, assert_noop};
use frame_benchmarking::{account, whitelisted_caller};
use crate::{mock::*, Error};
use frame_system;

type AccountId = <Test as frame_system::Config>::AccountId;

#[test]
fn should_init() {
	new_test_ext().execute_with(|| {
        let pool_address: AccountId = account("pool", 0, 1);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 9_000_000);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 1, 9_000_000);
		assert_ok!(Dex::init(Origin::signed(whitelisted_caller()), pool_address, 0,  9_000_000, 1, 9_000_000));
		assert_eq!(MultiToken::balances(0, pool_address), Some(9_000_000));
		assert_eq!(MultiToken::balances(1, pool_address), Some(9_000_000));
	});
}


#[test]
fn should_swap() {
	new_test_ext().execute_with(|| {
        let pool_address: AccountId = account("pool", 0, 1);
		let recepient: AccountId = account("recepient", 0, 1);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 9_000_000);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 1, 9_000_000);
		MultiToken::mint(Origin::signed(recepient), 0, 100_000);

		assert_ok!(Dex::init(Origin::signed(whitelisted_caller()), pool_address, 0,  9_000_000, 1, 9_000_000));
		assert_ok!(Dex::swap_token(Origin::signed(recepient), 0, 100_000));
		assert_eq!(MultiToken::balances(1, recepient), Some((100_000 * 9_000_000) / (9_000_000 + 100_000) * 97 / 100));
	});
}

#[test]
fn should_deposit() {
	new_test_ext().execute_with(|| {
        let pool_address: AccountId = account("pool", 0, 1);
		let recepient: AccountId = account("recepient", 0, 1);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 0, 9_000_000);
        MultiToken::mint(Origin::signed(whitelisted_caller()), 1, 9_000_000);
		MultiToken::mint(Origin::signed(recepient), 0, 100_000);
		MultiToken::mint(Origin::signed(recepient), 1, 100_000);

		assert_ok!(Dex::init(Origin::signed(whitelisted_caller()), pool_address, 0,  9_000_000, 1, 9_000_000));
		assert_ok!(Dex::deposit(Origin::signed(recepient), 100_000, 100_000));
		assert_eq!(MultiToken::balances(0, recepient), Some(9_100_000));
		assert_eq!(MultiToken::balances(1, recepient), Some(9_100_000));
	});
}

#[test]
fn should_withdraw_with_additional_liquidity() {
	new_test_ext().execute_with(|| {
        let pool_address: AccountId = account("pool", 0, 1);
		let recepient: AccountId = account("recepient", 0, 1);
		let second_recepient: AccountId = account("recepient2", 0, 1);

        MultiToken::mint_batch(Origin::signed(whitelisted_caller()), [0, 1].to_vec(), [9_000_000, 9_000_000].to_vec());
		MultiToken::mint_batch(Origin::signed(recepient), [0, 1].to_vec(), [1_000_000, 1_000_000].to_vec());
		MultiToken::mint_batch(Origin::signed(second_recepient), [0, 1].to_vec(), [1_000_000, 1_000_000].to_vec());
		let initial_liquidity: u64 = 1_000_000 * 1_000_000;
		assert_ok!(Dex::init(Origin::signed(whitelisted_caller()), pool_address, 0,  9_000_000, 1, 9_000_000));
		assert_ok!(Dex::deposit(Origin::signed(recepient), 1_000_000, 1_000_000));

		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 0, 100_000));
		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 1, 100_000));
		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 0, 100_000));
		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 1, 100_000));
		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 0, 100_000));
		assert_ok!(Dex::swap_token(Origin::signed(second_recepient), 1, 100_000));

		assert_ok!(Dex::withdraw(Origin::signed(recepient)));
		let first_token_balance = MultiToken::balances(0, recepient).unwrap();
		let second_token_balance = MultiToken::balances(1, recepient).unwrap();
		assert!(first_token_balance * second_token_balance > initial_liquidity)
	});
}

