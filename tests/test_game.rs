elrond_wasm::imports!();
elrond_wasm::derive_imports!();
use std::fmt::Debug;

use elrondcitygame::structs::{CitizenAttributes, BuildingType, BuildingRarity};
use elrondcitygame::vote::VoteModule;
use elrondcitygame::common::CommonModule;
use elrondcitygame::events::EventsModule;

use elrondcitygame::{self, GameContract, structs::Move};

use elrond_wasm_debug::{rust_biguint, DebugApi, tx_mock::TxInputESDT};

mod test_init;

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
struct DummyAttributes<M: ManagedTypeApi> {
    id: BigUint<M>
}

const GENESIS_TOKEN: &[u8] = b"GNS-6620e5";
const EXPANSION_TOKEN: &[u8] = b"EXP-6620e5";
const CITIZENTOKEN: &[u8] = b"CIT-6620e5";
const UNKNOWN_TOKEN: &[u8] = b"UNK-6620e5";
const ECITY_TOKEN: &[u8] = b"ECITY-6620e5";

#[test]
fn simple_vote() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Sets the user's ECITY token balance to 1000
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Send 1000 ECITY to the save_vote endpoint
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Check that the vote has been registered in the contract's storage
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let episode_number = sc.get_episode_number();
        let votes = sc.votes(episode_number).get(&0usize).unwrap();
        assert!(sc.has_voted(ManagedAddress::from_address(user_address)), "Vote not registered");
        assert_eq!(votes, BigUint::from(1000u64), "Vote not correctly registered");
    }).assert_ok();

    // Check that the user's ECITY balance is zero
    assert_eq!(b_wrapper.get_esdt_balance(&user_address, ECITY_TOKEN, 0u64), rust_biguint!(0u64), "User's ECITY balance is not zero");

    timestamp = 60 * 60 * 24 * 5;
    b_wrapper.set_block_timestamp(timestamp); // Set day to 5, to be able to claim your ecity

    // Check the vote results
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let winning_vote = sc.get_vote_result();
        assert_eq!(winning_vote, 0usize, "Vote result not correct");
    }).assert_ok();

    // Clears the votes
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.reclaim_vote();
    }).assert_ok();

    // Check that the vote has been cleared in the contract's storage
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        assert!(!sc.has_voted(ManagedAddress::from_address(user_address)), "Vote not cleared");
    }).assert_ok();

    // Check that the user's ECITY balance is 1000
    assert_eq!(b_wrapper.get_esdt_balance(&user_address, ECITY_TOKEN, 0u64), rust_biguint!(1000u64), "User did not get his ECITY back");
}

#[test]
fn get_votes_view() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Sets the user's ECITY token balance to 1000
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Send 1000 ECITY to the save_vote endpoint
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Check that the vote has been registered in the contract's storage
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let episode_number = sc.get_episode_number();
        let votes = sc.votes(episode_number).get(&0usize).unwrap();
        assert!(sc.has_voted(ManagedAddress::from_address(user_address)), "Vote not registered");
        assert_eq!(votes, BigUint::from(1000u64), "Vote not correctly registered");
    }).assert_ok();

    // Check that the user's ECITY balance is zero
    assert_eq!(b_wrapper.get_esdt_balance(&user_address, ECITY_TOKEN, 0u64), rust_biguint!(0u64), "User's ECITY balance is not zero");

    timestamp = 60 * 60 * 24 * 5;
    b_wrapper.set_block_timestamp(timestamp); // Set day to 5, to be able to claim your ecity

    // Check the get_votes view
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let votes = sc.votes(0u64);
        let votes = votes.get(&0usize).unwrap();
        assert_eq!(votes, BigUint::from(1000u64), "Vote not correctly registered");
    }).assert_ok();

    // Check the vote results
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let winning_vote = sc.get_vote_result();
        assert_eq!(winning_vote, 0usize, "Vote result not correct");
    }).assert_ok();

    // Clears the votes
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.reclaim_vote();
    }).assert_ok();

    // Check that the vote has been cleared in the contract's storage
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        assert!(!sc.has_voted(ManagedAddress::from_address(user_address)), "Vote not cleared");
    }).assert_ok();

    // Check that the user's ECITY balance is 1000
    assert_eq!(b_wrapper.get_esdt_balance(&user_address, ECITY_TOKEN, 0u64), rust_biguint!(1000u64), "User did not get his ECITY back");
}

#[test]
fn vote_too_late() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Sets the user's ECITY token balance to 1000
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Sets the block timestamp to 5 days after the start of the event (Voting period ended)
    timestamp += 60 * 60 * 24 * 5;
    b_wrapper.set_block_timestamp(timestamp);

    // Send 1000 ECITY to the save_vote endpoint
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_user_error("You can only vote on day 0");
}

#[test]
fn simple_citizen_stake() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper.set_nft_balance(
        user_address,
        CITIZENTOKEN,
        1u64,
        &rust_biguint!(1u64),
        &BoxedBytes::empty(),
    );

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();
}

#[test]
fn simple_citizen_stake_invalid_pos() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper.set_nft_balance(
        user_address,
        CITIZENTOKEN,
        1u64,
        &rust_biguint!(1u64),
        &BoxedBytes::empty(),
    );

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 6u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_user_error("Invalid pos");
}

#[test]
fn simple_stake_invalid_citizen_pos() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper.set_nft_balance(
        user_address,
        CITIZENTOKEN,
        1u64,
        &rust_biguint!(1u64),
        &BoxedBytes::empty(),
    );

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8, // Invalid pos
        });
        sc.perform_moves(moves);
    }).assert_user_error("Invalid citizen pos");
}

#[test]
fn invalid_collection() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;

    let timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Sets the user UNKNOWN balance to one NFT
    b_wrapper.set_nft_balance(
        user_address,
        UNKNOWN_TOKEN,
        1u64,
        &rust_biguint!(1u64),
        &BoxedBytes::empty(),
    );

    let transferts = [
        TxInputESDT {
            token_identifier: UNKNOWN_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(UNKNOWN_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_user_error("Unknown collection");
}

#[test]
fn invalid_collection_try_to_cheat() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user UNKNOWN balance to one NFT
    b_wrapper.set_nft_balance(
        user_address,
        UNKNOWN_TOKEN,
        1u64,
        &rust_biguint!(1u64),
        &BoxedBytes::empty(),
    );

    let transferts = [
        TxInputESDT {
            token_identifier: UNKNOWN_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN), // <-- try to cheat
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_user_error("Not enough token sent");
}

#[test]
fn stake_citizen_without_sending_tokens() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Generate new seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_user_error("Not enough token sent");
}

#[test]
fn stake_citizen_then_move_it() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Move the citizen
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 1u8,
            new_bat_pos: 2u8,
            new_cit_pos: 2u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

}

#[test]
fn stake_citizen_then_unstake_it() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Give 1000 ECITY to the user
    b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });



    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the contract's balance
    assert_eq!(b_wrapper.get_esdt_balance(sc_setup.contract_wrapper.address_ref(), CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Sets the block timestamp to day 6
    timestamp += 60 * 60 * 24 * 2;
    b_wrapper.set_block_timestamp(timestamp);

    // Move the citizen
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 1u8,
            new_bat_pos: 0u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is back in the user's balance
    assert_eq!(b_wrapper.get_esdt_balance(user_address, CITIZENTOKEN, 1u64), rust_biguint!(1u64));
}

#[test]
fn get_events_view() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Test the event view
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let events = sc.get_events();
        println!("events: {:?}", events);
        let event = events.get(0usize);
        assert_eq!(event.title, ManagedBuffer::from("0G Panic"));
        assert_eq!(event.description, ManagedBuffer::from("The first space-crime was just commited above Elrond City!"));
    }).assert_ok();
}

#[test]
fn nb_points_view() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the contract's balance
    assert_eq!(b_wrapper.get_esdt_balance(sc_setup.contract_wrapper.address_ref(), CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Sets the block timestamp to day 5
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    // Test the event view
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let points = sc.nb_points(&user_address.into());
        assert_eq!(points, BigUint::zero());
    }).assert_ok();

    // Move the citizen
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 1u8,
            new_bat_pos: 0u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is back in the user's balance
    assert_eq!(b_wrapper.get_esdt_balance(user_address, CITIZENTOKEN, 1u64), rust_biguint!(1u64));
}

#[test]
fn get_config_view() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the contract's balance
    assert_eq!(b_wrapper.get_esdt_balance(sc_setup.contract_wrapper.address_ref(), CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Sets the block timestamp to day 5
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    // Test the get_config view
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let config = sc.get_config();
        // Check that the config is correct
        assert_eq!(config.genesis_tokenidentifier, TokenIdentifier::from(GENESIS_TOKEN));
        assert_eq!(config.expansion_tokenidentifier, TokenIdentifier::from(EXPANSION_TOKEN));
        assert_eq!(config.citizen_tokenidentifier, TokenIdentifier::from(CITIZENTOKEN));
        assert_eq!(config.max_building, 3u8);
        assert_eq!(config.max_citizen, 2u8);
        assert_eq!(config.game_status, true);
        assert_eq!(config.start_timestamp, 1000u64);
        assert_eq!(config.day, 5u64);
        assert_eq!(config.episode_number, 0u64);
    }).assert_ok();
}

#[test]
fn stake_then_save_points() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.citizen_points(1u64).set(BigUint::from(1u64));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        )
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the contract's balance
    assert_eq!(b_wrapper.get_esdt_balance(sc_setup.contract_wrapper.address_ref(), CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Set the block timestamp to 1 hour later
    timestamp += 60 * 60 ;
    b_wrapper.set_block_timestamp(timestamp);

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let points = sc.user_points(0u64).get(&ManagedAddress::from(user_address));
        assert_eq!(points, None);
    }).assert_ok();

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.save_points();
    }).assert_ok();

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let points = sc.user_points(0u64).get(&ManagedAddress::from(user_address)).unwrap();
        assert_eq!(points, BigUint::from(12u64));
    }).assert_ok();
}

#[test]
fn stake_building_and_citizen() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user a Genesis NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Add a couple of minutes to the block timestamp
    timestamp += 60 * 2;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the user Citizen balance to one NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            CITIZENTOKEN,
            1u64,
            &rust_biguint!(1u64),
            &CitizenAttributes::<DebugApi> {
                id: 0,
                rank: 0,
                rarity_level: 1,
                job: ManagedBuffer::new_from_bytes(b"Job"),
                building: ManagedBuffer::new_from_bytes(b"Job"),
                face: ManagedBuffer::new_from_bytes(b"Job"),
                hair: ManagedBuffer::new_from_bytes(b"Job"),
                top: ManagedBuffer::new_from_bytes(b"Job"),
                over_clothe: ManagedBuffer::new_from_bytes(b"Job"),
                bottom: ManagedBuffer::new_from_bytes(b"Job"),
                hat: ManagedBuffer::new_from_bytes(b"Job"),
                held_item: ManagedBuffer::new_from_bytes(b"Job"),
                eyes_object: ManagedBuffer::new_from_bytes(b"Job"),
                mouth_object: ManagedBuffer::new_from_bytes(b"Job"),
                extra: ManagedBuffer::new_from_bytes(b"Job"),
            });
    });

    let transferts = [
        TxInputESDT {
            token_identifier: CITIZENTOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 1u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the contract's balance
    assert_eq!(b_wrapper.get_esdt_balance(sc_setup.contract_wrapper.address_ref(), CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Sets the block timestamp to day 5
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.save_points();
    }).assert_ok();

    // Wait for the next day and unstake all NFTs
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 0u8,
            new_bat_pos: 0u8,
            new_cit_pos: 0u8,
        });

        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(CITIZENTOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 1u8,
            new_bat_pos: 0u8,
            new_cit_pos: 0u8,
        });

        sc.perform_moves(moves);
    }).assert_ok();

    // Check that the citizen is in the user's balance
    assert_eq!(b_wrapper.get_esdt_balance(user_address, CITIZENTOKEN, 1u64), rust_biguint!(1u64));

    // Check that the Genesis NFT is in the user's balance
    assert_eq!(b_wrapper.get_esdt_balance(user_address, GENESIS_TOKEN, 1u64), rust_biguint!(1u64));
}

#[test]
fn add_complete_event_then_get_events() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Add buildingTypes to the event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        let mut building_types = ManagedVec::new();
        building_types.push(BuildingType::Police_Station);
        building_types.push(BuildingType::Rocket_Station);
        sc.add_building_types(BigUint::zero(), building_types);
    }).assert_ok();

    // Check getEvents view
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let events = sc.get_events();
        assert_eq!(events.len(), 1usize);
        let event = events.get(0usize);
        assert_eq!(event.title, ManagedBuffer::from("0G Panic"));
        assert_eq!(event.description, ManagedBuffer::from("The first space-crime was just commited above Elrond City!"));
        let effects = event.effects;
        let effect = effects.get(0usize);
        let activator = effect.activator.unwrap();
        assert!(activator.building_types.contains(&BuildingType::Police_Station));
    }).assert_ok();
}

#[test]
fn check_no_production_day_13() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Headquarter, BuildingRarity::GenesisClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user a Genesis NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Sets the block timestamp to day 13
    timestamp += 60 * 60 * 24 * 9;
    b_wrapper.set_block_timestamp(timestamp);

    // Check the user's points
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let points = sc.nb_points(&ManagedAddress::from(user_address));
        assert_ne!(points, BigUint::zero());
        print!("points before save: {:?}", points);
    }).assert_ok();

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.save_points();
    }).assert_ok();

    // Check the user's points
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let points = sc.nb_points(&ManagedAddress::from(user_address));
        print!("points after save: {:?}", points);
        assert_eq!(points, BigUint::zero());
    }).assert_ok();

    // Sets the block timestamp to day 14
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    // Check the user's points
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let points_later = sc.nb_points(&ManagedAddress::from(user_address));
        print!("points later: {:?}", points_later);
        assert_eq!(points_later, BigUint::zero());
    }).assert_ok();
}

#[test]
fn claim_previous_episode() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Headquarter, BuildingRarity::GenesisClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Give 1000 ECITY to the owner
    b_wrapper.set_esdt_balance(
        owner_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Set the owner as the router
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.router_address().set(&ManagedAddress::from(owner_address));
    }).assert_ok();

    // Depositing this Episode's ECITY
    b_wrapper.execute_esdt_transfer(&owner_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.deposit_ecity();
    }).assert_ok();

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user a Genesis NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Sets the block timestamp to day 12
    timestamp += 60 * 60 * 24 * 8;
    b_wrapper.set_block_timestamp(timestamp);

    // Check the user's points
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let managed_user_address = &ManagedAddress::from(user_address);
        let points = sc.nb_points(managed_user_address);
        //assert_ne!(points, BigUint::zero());
    }).assert_ok();

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.save_points();
    }).assert_ok();

    // Wait until the next episode
    timestamp += 60 * 60 * 24 * 5;
    // Sets the block timestamp to day 17, or day 3 of the next episode

    b_wrapper.set_block_timestamp(timestamp);

    // Check the user's claimable ECITY
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let claimable = sc.all_claimable_ecity(&ManagedAddress::from_address(user_address));
        assert_ne!(claimable, BigUint::zero());
    }).assert_ok();
}

#[test]
fn stake_two_buildings_then_swap_them() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Headquarter, BuildingRarity::GenesisClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Give 1000 ECITY to the owner
    b_wrapper.set_esdt_balance(
        owner_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Set the owner as the router
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.router_address().set(&ManagedAddress::from(owner_address));
    }).assert_ok();

    // Depositing this Episode's ECITY
    b_wrapper.execute_esdt_transfer(&owner_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.deposit_ecity();
    }).assert_ok();

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 2
    timestamp += 60 * 60 * 24 * 2;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user two Genesis NFTs
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            2u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        },
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 2u64,
            value: rust_biguint!(1u64),
        },
    ];

    // Place Genesis 1 at position 1 0 and Genesis 2 at position 2 0
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 2u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 2u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Move the timestamp to day 3
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    // Swap the two Genesis
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &[], |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 1u8,
            old_cit_pos: 0u8,
            new_bat_pos: 2u8,
            new_cit_pos: 0u8,
        });
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 2u64,
            old_bat_pos: 2u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();
}

#[test]
fn stake_then_stake_at_the_same_place() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Headquarter, BuildingRarity::GenesisClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Give 1000 ECITY to the owner
    b_wrapper.set_esdt_balance(
        owner_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Set the owner as the router
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.router_address().set(&ManagedAddress::from(owner_address));
    }).assert_ok();

    // Depositing this Episode's ECITY
    b_wrapper.execute_esdt_transfer(&owner_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.deposit_ecity();
    }).assert_ok();

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 2
    timestamp += 60 * 60 * 24 * 2;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user two Genesis NFTs
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            2u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        }
    ];

    // Place Genesis 1 at position 1 0
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });

        sc.perform_moves(moves);
    }).assert_ok();

    // Move the timestamp to day 3
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 2u64,
            value: rust_biguint!(1u64),
        }
    ];

    // Place Genesis 2 at position 1 0
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 2u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_user_error("Slot not empty");
}

#[test]
fn stake_boosted_building() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper; 

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Add building type Headquarter to the event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_building_types(BigUint::zero(), ManagedVec::from(vec![BuildingType::Rocket_Station, BuildingType::Rocket_Station, BuildingType::Rocket_Station]));
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Rocket_Station, BuildingRarity::ExpansionClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 4
    timestamp += 60 * 60 * 24 * 4;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user a Genesis NFT
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            EXPANSION_TOKEN,
            22u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: EXPANSION_TOKEN.to_vec(),
            nonce: 22u64,
            value: rust_biguint!(1u64),
        }
    ];

    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(EXPANSION_TOKEN),
            nonce: 22u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Sets the block timestamp to day 13
    timestamp += 60 * 60 * 24 * 9;
    b_wrapper.set_block_timestamp(timestamp);

    /*// Check the user's points
    b_wrapper.execute_query(&sc_setup.contract_wrapper, |sc| {
        let points = sc.nb_points(&ManagedAddress::from(user_address));
        assert_ne!(points, BigUint::zero());
        print!("points: {:?}", points);
    }).assert_ok();*/

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.save_points();
    }).assert_ok();
}

#[test]
fn stake_three_then_rotate_them() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;
    let mut sc_setup2 = test_init::setup_contract(elrondcitygame::contract_obj);
    let b_wrapper2 = &mut sc_setup2.blockchain_wrapper;

    let mut timestamp = 1000u64;
    b_wrapper.set_block_timestamp(timestamp);

    // Sets the contract's tokenidentifiers and parameters
    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_genesis_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(GENESIS_TOKEN));
        sc.set_expansion_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(EXPANSION_TOKEN));
        sc.set_citizen_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(CITIZENTOKEN));
        sc.set_ecity_tokenidentifier(elrond_wasm::types::TokenIdentifier::from(ECITY_TOKEN));
        sc.nb_events().set(1usize);
        sc.start();
    }).assert_ok();

    // Change the seeds
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.change_seed();
    }).assert_ok();

    // Add a base event
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.add_event(
            ManagedBuffer::from("0G Panic"),
            ManagedBuffer::from("The first space-crime was just commited above Elrond City!"),
            BigUint::zero()
        );
    }).assert_ok();

    // Set points for Headquarter
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_building_points_separately(BuildingType::Headquarter, BuildingRarity::GenesisClassic, BigUint::from(100u64));
    }).assert_ok();

     // Give 1000 ECITY to the user
     b_wrapper.set_esdt_balance(
        user_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Give 1000 ECITY to the owner
    b_wrapper.set_esdt_balance(
        owner_address,
        ECITY_TOKEN,
        &rust_biguint!(1000u64),
    );

    // Set the owner as the router
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.router_address().set(&ManagedAddress::from(owner_address));
    }).assert_ok();

    // Depositing this Episode's ECITY
    b_wrapper.execute_esdt_transfer(&owner_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.deposit_ecity();
    }).assert_ok();

    // User votes for the event
    b_wrapper.execute_esdt_transfer(&user_address, &sc_setup.contract_wrapper, ECITY_TOKEN, 0u64, &rust_biguint!(1000u64), |sc| {
        sc.vote(0usize);
    }).assert_ok();

    // Sets the block timestamp to day 2
    timestamp += 60 * 60 * 24 * 2;
    b_wrapper.set_block_timestamp(timestamp);

    // Give the user three Genesis NFTs
    b_wrapper2.execute_in_managed_environment(|| {
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            1u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            2u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
        b_wrapper.set_nft_balance(
            user_address,
            GENESIS_TOKEN,
            3u64,
            &rust_biguint!(1u64),
            &BoxedBytes::empty(),
        );
    });

    let transferts = [
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 1u64,
            value: rust_biguint!(1u64),
        },
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 2u64,
            value: rust_biguint!(1u64),
        },
        TxInputESDT {
            token_identifier: GENESIS_TOKEN.to_vec(),
            nonce: 3u64,
            value: rust_biguint!(1u64),
        },
    ];

    // Place Genesis 1 at position 1 0, Genesis 2 at position 2 0, and Genesis 3 at position 3 0
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 1u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 1u8,
            new_cit_pos: 0u8,
        });
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 2u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 2u8,
            new_cit_pos: 0u8,
        });
        moves.push(Move::<DebugApi> {
            token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
            nonce: 3u64,
            old_bat_pos: 0u8,
            old_cit_pos: 0u8,
            new_bat_pos: 3u8,
            new_cit_pos: 0u8,
        });
        sc.perform_moves(moves);
    }).assert_ok();

    // Move the timestamp to day 3
    timestamp += 60 * 60 * 24 * 1;
    b_wrapper.set_block_timestamp(timestamp);

    let transferts = [];

    // Rotate all NFTs. Move Genesis 1 to position 2 0, Genesis 2 to position 3 0, and Genesis 3 to position 1 0
    b_wrapper.execute_esdt_multi_transfer(&user_address, &sc_setup.contract_wrapper, &transferts, |sc| {
        let mut moves = MultiValueManagedVec::new();
        moves.push(
            Move::<DebugApi> {
                token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
                nonce: 1u64,
                old_bat_pos: 1u8,
                old_cit_pos: 0u8,
                new_bat_pos: 2u8,
                new_cit_pos: 0u8,
            }
        );
        moves.push(
            Move::<DebugApi> {
                token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
                nonce: 2u64,
                old_bat_pos: 2u8,
                old_cit_pos: 0u8,
                new_bat_pos: 3u8,
                new_cit_pos: 0u8,
            }
        );
        moves.push(
            Move::<DebugApi> {
                token_identifier: TokenIdentifier::from(GENESIS_TOKEN),
                nonce: 3u64,
                old_bat_pos: 3u8,
                old_cit_pos: 0u8,
                new_bat_pos: 1u8,
                new_cit_pos: 0u8,
            }
        );
        sc.perform_moves(moves);
    }).assert_ok();
}
