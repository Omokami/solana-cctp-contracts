//! Events

use anchor_lang::prelude::*;

#[event]
pub struct OwnershipTransferStarted {
    pub previous_owner: Pubkey,
    pub new_owner: Pubkey,
}

#[event]
pub struct OwnershipTransferred {
    pub previous_owner: Pubkey,
    pub new_owner: Pubkey,
}

#[event]
pub struct DepositForBurn {
    pub nonce: u64,
    pub burn_token: Pubkey,
    pub amount: u64,
    pub depositor: Pubkey,
    pub mint_recipient: Vec<u8>,
    pub destination_domain: u32,
    pub destination_token_messenger: Vec<u8>,
    pub destination_caller: Vec<u8>,
}

#[event]
pub struct MintAndWithdraw {
    pub mint_recipient: Pubkey,
    pub amount: u64,
    pub mint_token: Pubkey,
}

#[event]
pub struct RemoteTokenMessengerAdded {
    pub domain: Pubkey,
    pub token_messenger: Vec<u8>,
}

#[event]
pub struct RemoteTokenMessengerRemoved {
    pub domain: Pubkey,
    pub token_messenger: Vec<u8>,
}