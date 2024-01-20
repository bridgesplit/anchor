use anchor_lang::Result;
use spl_token_2022::{extension::ExtensionType, state::Mint};

pub fn find_mint_account_size(
    token_metadata: Option<&TokenMetadataInitializeArgs>,
    token_group_data: Option<&TokenGroupInitializeArgs>,
    token_group_member_data: Option<bool>,
    metadata_pointer_data: Option<&MetadataPointerInitializeArgs>,
    group_pointer_data: Option<&GroupPointerInitializeArgs>,
    group_member_pointer_data: Option<&GroupMemberPointerInitializeArgs>,
) -> Result<usize> {
    let mut extension_types = Vec::new();
    if let Some(_) = token_metadata {
        extension_types.push(ExtensionType::TokenMetadata);
    }
    if let Some(_) = token_group_data {
        extension_types.push(ExtensionType::TokenGroup);
    }
    if let Some(_) = token_group_member_data {
        extension_types.push(ExtensionType::TokenGroupMember);
    }
    if let Some(_) = metadata_pointer_data {
        extension_types.push(ExtensionType::MetadataPointer);
    }
    if let Some(_) = group_pointer_data {
        extension_types.push(ExtensionType::GroupPointer);
    }
    if let Some(_) = group_member_pointer_data {
        extension_types.push(ExtensionType::GroupMemberPointer);
    }
    Ok(ExtensionType::try_calculate_account_len::<Mint>(
        &extension_types,
    )?)
}

pub mod group_member_pointer;
pub mod group_pointer;
pub mod metadata_pointer;
pub mod token_group;
pub mod token_metadata;

pub use group_member_pointer::*;
pub use group_pointer::*;
pub use metadata_pointer::*;
pub use token_group::*;
pub use token_metadata::*;
