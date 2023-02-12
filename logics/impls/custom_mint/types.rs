use openbrush::{
    storage::Mapping,
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
    },
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

pub type TokenId = Id;

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub last_token_id: u64,
    pub royalty: Mapping<TokenId,u16>,
    pub creator: AccountId,
    pub token_uri: Mapping<TokenId,String>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TokenError {
    OutOfBoundRoyaltyValue,
}

impl TokenError {
    pub fn as_str(&self) -> String {
        match self {
            TokenError::OutOfBoundRoyaltyValue => String::from("OutOfBoundRoyaltyValue"),
        }
    }
}