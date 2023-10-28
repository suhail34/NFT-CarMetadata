use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	CarMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[writable]` car: [CarMetadata] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 4. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 5. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 6. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 9. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 10. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - color: [String] 
/// - model: [String] 
/// - short_description: [String] 
pub fn mint(
	program_id: &Pubkey,
	for_initialize_mint_2: &[&AccountInfo],
	for_create: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	for_set_authority: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	car: &mut AccountPDA<CarMetadata>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	owner: &AccountInfo,
	color: String,
	model: String,
	short_description: String,
) -> ProgramResult {
    // Implement your business logic here...
    car.data.color = color;
    car.data.model = model;
    car.data.short_description = short_description;
    car.data.mint = *mint.info.key;
    car.data.assoc_account = Some(*assoc_token_account.key);


	csl_spl_token::src::cpi::initialize_mint_2(
		for_initialize_mint_2,
		Default::default(),
		Pubkey::default(),
		Some(Pubkey::default()),
	)?;

	csl_spl_assoc_token::src::cpi::create(
		for_create,
	)?;

	csl_spl_token::src::cpi::mint_to(
		for_mint_to,
		Default::default(),
	)?;

	csl_spl_token::src::cpi::set_authority(
		for_set_authority,
		Default::default(),
		Some(Pubkey::default()),
	)?;


    Ok(())
}