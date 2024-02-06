use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Result;
use anchor_lang::{context::CpiContext, Accounts};

use spl_token_2022::solana_zk_token_sdk::zk_token_elgamal::pod::ElGamalPubkey;

pub struct ConfidentialTransferInitializeArgs {
    pub authority: Option<Pubkey>,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: Option<ElGamalPubkey>,
}

pub fn confidential_transfer_initialize<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, ConfidentialTransferInitialize<'info>>,
    args: ConfidentialTransferInitializeArgs,
) -> Result<()> {
    let ix = spl_token_2022::extension::confidential_transfer::instruction::initialize_mint(
        ctx.accounts.token_program_id.key,
        ctx.accounts.mint.key,
        args.authority,
        args.auto_approve_new_accounts,
        args.auditor_elgamal_pubkey,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.token_program_id, ctx.accounts.mint],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ConfidentialTransferInitialize<'info> {
    pub token_program_id: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}
