use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Result;
use anchor_lang::{context::CpiContext, Accounts};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct TransferHookInitializeArgs {
    pub authority: Option<Pubkey>,
    pub transfer_hook_program_id: Option<Pubkey>,
}

pub fn transfer_hook_initialize<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, TransferHookInitialize<'info>>,
    args: TransferHookInitializeArgs,
) -> Result<()> {
    let ix = spl_token_2022::extension::transfer_hook::instruction::initialize(
        ctx.accounts.token_program_id.key,
        ctx.accounts.mint.key,
        args.authority,
        args.transfer_hook_program_id,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.token_program_id, ctx.accounts.mint],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct TransferHookInitialize<'info> {
    pub token_program_id: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
}
