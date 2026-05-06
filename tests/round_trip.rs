#![allow(deprecated)]

use {
    solana_hash::Hash,
    solana_message::{compiled_instruction::CompiledInstruction, Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::{Reward, RewardType},
    yellowstone_grpc_convert::{convert_from, convert_to},
};

#[test]
fn round_trip_transaction_legacy() {
    let pubkey = Pubkey::new_unique();
    let to_pubkey = Pubkey::new_unique();
    let recent_blockhash = Hash::new_unique();

    let ix = CompiledInstruction {
        program_id_index: 0,
        accounts: vec![0, 1],
        data: vec![1, 2, 3, 4],
    };
    let message = Message {
        header: solana_message::MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![pubkey, to_pubkey],
        recent_blockhash,
        instructions: vec![ix],
    };
    let tx = VersionedTransaction {
        signatures: vec![Signature::from([1u8; 64])],
        message: VersionedMessage::Legacy(message),
    };

    let proto_tx = convert_to::create_transaction(&tx);
    let decoded_tx = convert_from::create_tx_versioned(proto_tx).unwrap();

    assert_eq!(tx.signatures, decoded_tx.signatures);
    assert_eq!(tx.message, decoded_tx.message);
}

#[test]
fn round_trip_reward() {
    let reward = Reward {
        pubkey: Pubkey::new_unique().to_string(),
        lamports: 42,
        post_balance: 1000,
        reward_type: Some(RewardType::Staking),
        commission: Some(10),
    };

    let proto_reward = convert_to::create_reward(&reward);
    let decoded_reward = convert_from::create_reward(proto_reward).unwrap();

    assert_eq!(reward, decoded_reward);
}

#[test]
fn round_trip_rewards_obj() {
    let rewards = vec![Reward {
        pubkey: Pubkey::new_unique().to_string(),
        lamports: 100,
        post_balance: 5000,
        reward_type: Some(RewardType::Fee),
        commission: None,
    }];
    let num_partitions = Some(4u64);

    let proto_rewards = convert_to::create_rewards_obj(&rewards, num_partitions);
    let decoded = convert_from::create_rewards_obj(proto_rewards).unwrap();

    assert_eq!(rewards, decoded.rewards);
    assert_eq!(num_partitions, decoded.num_partitions);
}
