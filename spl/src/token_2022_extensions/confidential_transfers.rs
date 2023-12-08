use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Result;
use anchor_lang::{context::CpiContext, Accounts};

use borsh::{BorshDeserialize, BorshSerialize};
use spl_token_2022::solana_zk_token_sdk::zk_token_elgamal::pod::ElGamalPubkey;

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct ConfidentialTransferIntializeMintArgs {
    pub authority: Option<Pubkey>,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: Option<[u8; 32]>,
}

pub fn confidential_transfer_initialize_mint<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, ConfidentialTransferIntializeMint<'info>>,
    args: ConfidentialTransferIntializeMintArgs,
) -> Result<()> {
    let elgamal_pubkey = match args.auditor_elgamal_pubkey {
        Some(pubkey) => Some(ElGamalPubkey(pubkey)),
        None => None,
    };
    let ix = spl_token_2022::extension::confidential_transfer::instruction::initialize_mint(
        ctx.accounts.token_program_id.key,
        ctx.accounts.mint.key,
        args.authority,
        args.auto_approve_new_accounts,
        elgamal_pubkey,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.token_program_id, ctx.accounts.mint],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ConfidentialTransferIntializeMint<'info> {
    pub token_program_id: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}
