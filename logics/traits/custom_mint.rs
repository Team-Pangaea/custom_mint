use ink::prelude::string::String as PreludeString;

use openbrush::{
    contracts::{
        psp34::Id,
        psp34::PSP34Error,
    },
    traits::{
        AccountId,
        String,
    },
};

#[openbrush::wrapper]
pub type CustomMintRef = dyn CustomMint;

#[openbrush::trait_definition]
pub trait CustomMint {
    #[ink(message)]
    fn mint(&mut self, to: AccountId, token_uri: String, marketplace: AccountId) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;
    #[ink(message)]
    fn get_token_uri(&mut self, token_id: Id) -> Result<PreludeString, PSP34Error>;
    
}