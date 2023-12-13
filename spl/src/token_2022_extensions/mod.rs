use spl_token_2022::{state::Mint, extension::ExtensionType};
use anchor_lang::Result;

pub fn find_mint_account_size(
    metadata_pointer_data: Option<&MetadataPointerInitializeArgs>
) -> Result<usize> {
    let mut extension_types = Vec::new();
    if let Some(_) = metadata_pointer_data {
        extension_types.push(ExtensionType::MetadataPointer);
    }
    Ok(ExtensionType::try_calculate_account_len::<Mint>(&extension_types)?)
}

pub mod metadata_pointer;

pub use metadata_pointer::*;
