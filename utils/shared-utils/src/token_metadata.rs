use anchor_lang::{prelude::*, solana_program};
use mpl_token_metadata::{
  state::{CollectionDetails, DataV2},
  ID,
};

use anchor_spl::token::{self, ThawAccount};
#[derive(Accounts)]
pub struct FreezeAccountContext<'info> {
    pub token_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub freeze_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn freeze_account<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, FreezeAccountContext<'info>>
) -> Result<()> {
  let cpi_accounts = token::FreezeAccount {
      account: ctx.accounts.token_account.to_account_info(),
      mint: ctx.accounts.mint.to_account_info(),
      authority: ctx.accounts.freeze_authority.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::freeze_account(cpi_ctx)
}
#[derive(Accounts)]
pub struct ThawAccountContext<'info> {
  pub token_account: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub freeze_authority: AccountInfo<'info>,
}

pub fn thaw_account<'info>(ctx: CpiContext<'_, '_, '_, 'info, ThawAccountContext<'info>>) -> Result<()> {

  let cpi_accounts = ThawAccount {
      account: ctx.accounts.token_account.to_account_info(),
      mint: ctx.accounts.mint.to_account_info(),
      authority: ctx.accounts.freeze_authority.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_program.to_account_info();

  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts)
    .with_signer(ctx.signer_seeds);

  token::thaw_account(cpi_ctx)
}

#[derive(Accounts)]
pub struct SignMetadata<'info> {
  pub metadata: AccountInfo<'info>,
  pub creator: AccountInfo<'info>,
}

pub fn sign_metadata<'info>(ctx: CpiContext<'_, '_, '_, 'info, SignMetadata<'info>>) -> Result<()> {
  let ix = mpl_token_metadata::instruction::sign_metadata(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.creator.key,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMetadataAccountsV3<'info> {
  pub metadata: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub mint_authority: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn create_metadata_accounts_v3<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>>,
  data: DataV2,
  is_mutable: bool,
  update_authority_is_signer: bool,
  details: Option<CollectionDetails>,
) -> Result<()> {
  let DataV2 {
    name,
    symbol,
    uri,
    creators,
    seller_fee_basis_points,
    collection,
    uses,
  } = data;
  let ix = mpl_token_metadata::instruction::create_metadata_accounts_v3(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.mint_authority.key,
    *ctx.accounts.payer.key,
    *ctx.accounts.update_authority.key,
    name,
    symbol,
    uri,
    creators,
    seller_fee_basis_points,
    update_authority_is_signer,
    is_mutable,
    collection,
    uses,
    details,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
  fn id() -> Pubkey {
    ID
  }
}

#[derive(Accounts)]
pub struct CreateMasterEditionV3<'info> {
  pub edition: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub mint_authority: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub metadata: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn create_master_edition_v3<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, CreateMasterEditionV3<'info>>,
  max_supply: Option<u64>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::create_master_edition_v3(
    ID,
    *ctx.accounts.edition.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.update_authority.key,
    *ctx.accounts.mint_authority.key,
    *ctx.accounts.metadata.key,
    *ctx.accounts.payer.key,
    max_supply,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
  pub payer: AccountInfo<'info>,
  pub metadata: AccountInfo<'info>,
  pub collection_authority: AccountInfo<'info>,
  pub collection_mint: AccountInfo<'info>,
  pub collection_metadata: AccountInfo<'info>,
  pub collection_master_edition: AccountInfo<'info>,
}

pub fn verify_collection<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, VerifyCollection<'info>>,
  collection_authority_record: Option<Pubkey>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::verify_collection(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.collection_authority.key,
    *ctx.accounts.payer.key,
    *ctx.accounts.collection_mint.key,
    *ctx.accounts.collection_metadata.key,
    *ctx.accounts.collection_master_edition.key,
    collection_authority_record,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct VerifySizedCollectionItem<'info> {
  pub payer: AccountInfo<'info>,
  pub metadata: AccountInfo<'info>,
  pub collection_authority: AccountInfo<'info>,
  pub collection_mint: AccountInfo<'info>,
  pub collection_metadata: AccountInfo<'info>,
  pub collection_master_edition: AccountInfo<'info>,
}

pub fn verify_sized_collection_item<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, VerifySizedCollectionItem<'info>>,
  collection_authority_record: Option<Pubkey>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::verify_sized_collection_item(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.collection_authority.key,
    *ctx.accounts.payer.key,
    *ctx.accounts.collection_mint.key,
    *ctx.accounts.collection_metadata.key,
    *ctx.accounts.collection_master_edition.key,
    collection_authority_record,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}


#[derive(Accounts)]
pub struct FreezeDelegatedAccount<'info> {
  pub metadata: AccountInfo<'info>,
  pub delegate: AccountInfo<'info>,
  pub token_account: AccountInfo<'info>,
  pub edition: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

pub fn freeze_delegated_account<'info>(
  ctx: CpiContext<'_, '_, '_, 'info,
  FreezeDelegatedAccount<'info>>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::freeze_delegated_account(
    ID,
    *ctx.accounts.delegate.key,
    *ctx.accounts.token_account.key,
    *ctx.accounts.edition.key,
    *ctx.accounts.mint.key,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ThawDelegatedAccount<'info> {
  pub metadata: AccountInfo<'info>,
  pub delegate: AccountInfo<'info>,
  pub token_account: AccountInfo<'info>,
  pub edition: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

pub fn thaw_delegated_account<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, ThawDelegatedAccount<'info>>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::thaw_delegated_account(
    ID,
    *ctx.accounts.delegate.key,
    *ctx.accounts.token_account.key,
    *ctx.accounts.edition.key,
    *ctx.accounts.mint.key,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
  pub metadata: AccountInfo<'info>,
  pub owner: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub token: AccountInfo<'info>,
  pub edition: AccountInfo<'info>,
  pub spl_token: AccountInfo<'info>,
  pub collection_metadata: AccountInfo<'info>,
}

pub fn burn_nft<'info>(ctx: CpiContext<'_, '_, '_, 'info, BurnNft<'info>>) -> Result<()> {
  let ix = mpl_token_metadata::instruction::burn_nft(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.owner.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.token.key,
    *ctx.accounts.edition.key,
    *ctx.accounts.spl_token.key,
    Some(*ctx.accounts.collection_metadata.key),
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdateMetadataAccountsV2<'info> {
  pub metadata: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
}

pub fn update_metadata_accounts_v2<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, UpdateMetadataAccountsV2<'info>>,
  new_update_authority: Option<Pubkey>,
  data: Option<DataV2>,
  primary_sale_happened: Option<bool>,
  is_mutable: Option<bool>,
) -> Result<()> {
  let ix = mpl_token_metadata::instruction::update_metadata_accounts_v2(
    ID,
    *ctx.accounts.metadata.key,
    *ctx.accounts.update_authority.key,
    new_update_authority,
    data,
    primary_sale_happened,
    is_mutable,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}
