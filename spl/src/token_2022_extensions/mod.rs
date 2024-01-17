use spl_token_2022::{state::Mint, extension::ExtensionType};
use anchor_lang::Result;

pub fn find_mint_account_size(
    metadata_pointer_data: Option<&MetadataPointerInitializeArgs>,
    group_pointer_data: Option<&GroupPointerInitializeArgs>,
    group_member_pointer_data: Option<&GroupMemberPointerInitializeArgs>,
) -> Result<usize> {
    let mut extension_types = Vec::new();
    if let Some(_) = metadata_pointer_data {
        extension_types.push(ExtensionType::MetadataPointer);
    }
    if let Some(_) = group_pointer_data {
        extension_types.push(ExtensionType::GroupPointer);
    }
    if let Some(_) = group_member_pointer_data {
        extension_types.push(ExtensionType::GroupMemberPointer);
    }
    Ok(ExtensionType::try_calculate_account_len::<Mint>(&extension_types)?)
}

pub mod metadata_pointer;
pub mod group_member_pointer;
pub mod group_pointer;

pub use metadata_pointer::*;
pub use group_member_pointer::*;
pub use group_pointer::*;
