use anchor_lang::prelude::*;

use crate::state::*;
use anchor_lang::solana_program::sysvar::rent::Rent;
use anchor_spl::associated_token::AssociatedToken;
use mpl_bubblegum::state::metaplex_anchor::MplTokenMetadata;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, ThawAccount};

#[derive(Accounts)]
#[instruction(rns_id: String, wallet: Pubkey)]
pub struct BurnNonTransferableNft<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,

  #[account(
    mut,
    associated_token::mint = non_transferable_nft_mint,
    associated_token::authority = authority,
  )]
  pub user_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    constraint = non_transferable_user_status.authority == authority.key(),
    seeds = [
      NON_TRANSFERABLE_NFT_USERSTATUS_PREFIX.as_ref(),
      &hash_seed(&rns_id.clone())[..32],
      wallet.key().as_ref()
    ],
    bump = non_transferable_user_status.bump
  )]
  pub non_transferable_user_status: Box<Account<'info, UserStatusAccount>>,

  #[account(
    mut,
    constraint = non_transferable_rns_id_status.authority == non_transferable_project.authority.key(),
    seeds = [
        NON_TRANSFERABLE_NFT_RNSID_PREFIX.as_ref(),
        &hash_seed(&rns_id)[..32],
    ],
    bump
  )]
  pub non_transferable_rns_id_status: Box<Account<'info, RnsIdStatusAccount>>,


  #[account(
    mut,
    constraint = non_transferable_nft_status.authority == authority.key(),
    seeds = [
      NON_TRANSFERABLE_NFT_STATUS_PREFIX.as_ref(),
      non_transferable_nft_mint.key().as_ref()
    ],
    bump
  )]
  pub non_transferable_nft_status: Box<Account<'info, NftStatusAccount>>,

  #[account(
    mut,
    seeds = [NON_TRANSFERABLE_PROJECT_PREFIX.as_ref()],
    bump
  )]
  pub non_transferable_project: Box<Account<'info, ProjectAccount>>,

  /// CHECK: Used in CPI So no Harm
  #[account()]
  pub non_transferable_project_mint: UncheckedAccount<'info>,

  /// CHECK: Used in CPI So no Harm
  #[account(
    mut,
    seeds = [
      "metadata".as_ref(),
      token_metadata_program.key().as_ref(),
      non_transferable_project_mint.key().as_ref()
    ],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub non_transferable_project_metadata: AccountInfo<'info>,

  #[account(mut)]
  pub non_transferable_nft_mint: Box<Account<'info, Mint>>,

  /// CHECK: Used in CPI
  #[account(
    mut,
    seeds = [
      "metadata".as_ref(),
      token_metadata_program.key().as_ref(),
      non_transferable_nft_mint.key().as_ref()
    ],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub non_transferable_nft_metadata: AccountInfo<'info>,

  /// CHECK: Used in CPI
  #[account(
    mut,
    seeds = [
      "metadata".as_ref(),
      token_metadata_program.key().as_ref(),
      non_transferable_nft_mint.key().as_ref(),
      "edition".as_ref()
    ],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub non_transferable_nft_master_edition: AccountInfo<'info>,

  pub token_metadata_program: Program<'info, MplTokenMetadata>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<BurnNonTransferableNft>, rns_id: String, wallet: Pubkey) -> Result<()> {
  msg!("start burn ..");

  let signer_seeds: &[&[u8]] = &[
    NON_TRANSFERABLE_PROJECT_PREFIX.as_bytes(),
    &[ctx.accounts.non_transferable_project.bump],
  ];

  msg!("thaw_account");

    let cpi_accounts = ThawAccount {
      account: ctx.accounts.user_token_account.to_account_info(),
      mint: ctx.accounts.non_transferable_nft_mint.to_account_info(),
      authority: ctx.accounts.non_transferable_project.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::thaw_account(
    cpi_ctx.with_signer(&[&signer_seeds[..]]),
  )?;

  msg!("burn_nft");

  // 烧毁NFT
  let cpi_accounts = Burn {
    authority: ctx.accounts.authority.to_account_info().clone(),
    from: ctx.accounts.user_token_account.to_account_info().clone(),
    mint: ctx
      .accounts
      .non_transferable_nft_mint
      .to_account_info()
      .clone(),
  };

  let cpi_program = ctx.accounts.token_program.clone();

  let _ctx = CpiContext::new(cpi_program.to_account_info(), cpi_accounts);

  let _ = token::burn(_ctx, 1);



  let status = &mut ctx.accounts.non_transferable_nft_status;
  status.merkle_root = String::new();
  status.rns_id = String::new();

  let status = &mut ctx.accounts.non_transferable_user_status;
  status.is_authorized = false;
  status.is_minted = false;


  msg!(
    "RNSBurnID:_rnsId:{};_wallet:{};_tokenId:{}",
    rns_id.clone(),
    ctx.accounts.authority.to_account_info().key(),
    ctx.accounts.non_transferable_nft_mint.key().to_string()
  );

  emit!(BurnEvent {
    rns_id: rns_id.clone(),
    wallet: ctx.accounts.authority.to_account_info().key(),
    token_id: rns_id.clone()
  });

  Ok(())
}

#[event]
pub struct BurnEvent {
  pub rns_id: String,
  pub wallet: Pubkey,
  pub token_id: String,
}
