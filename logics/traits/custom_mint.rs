use ink_prelude::string::String as PreludeString;

use openbrush::{
    contracts::{
        psp34::Id,
        psp34::PSP34Error,
        psp34::extensions::enumerable::*
    },
    traits::{
        AccountId,
        Balance,
        String,
    },
};

#[openbrush::wrapper]
pub type TokenRef = dyn CustomMint;

#[openbrush::trait_definition]
pub trait CustomMint {
    #[ink(message)]
    fn mint(&mut self, to: AccountId, token_uri: String, royalty: u16) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn get_token_uri(&mut self, token_id: Id) -> Result<PreludeString, PSP34Error>;
    #[ink(message)]
    fn get_token_royalty(&mut self, token_id: Id) -> Result<u16, PSP34Error>;
    #[ink(message)]
    fn get_royalty_info(&mut self, token_id: Id, price: Balance) -> Result<(Balance,AccountId),PSP34Error>;
}