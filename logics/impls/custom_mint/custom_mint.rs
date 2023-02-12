use ink_prelude::string::{
    String as PreludeString,
    ToString,
};

use crate::impls::custom_mint::types::{
    Data,
    TokenError,
};

pub use crate::traits::custom_mint::CustomMint;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::Id,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};

pub trait Internal {
    fn token_exists(&self, id: Id) -> Result<(), PSP34Error>;

    fn check_royalty(&self, royalty: u16) -> Result<(),PSP34Error>;
}

impl<T> CustomMint for T
where
    T: Storage<Data>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    default fn mint(&mut self, to: AccountId, token_uri: String, royalty: u16) -> Result<(), PSP34Error> {
        self.check_royalty(royalty)?;
        let mint_id = self.data::<Data>().last_token_id + 1;

        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(to.clone(), Id::U64(mint_id))?;
        self.data::<Data>().last_token_id += 1;
        self.data::<Data>().royalty.insert(&Id::U64(mint_id),&royalty);
        self.data::<Data>().token_uri.insert(&Id::U64(mint_id),&token_uri);
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, String::from("baseUri"), uri.into_bytes());
        Ok(())
    }

    default fn get_token_uri(&mut self, token_id: Id) -> Result<PreludeString, PSP34Error> {
        self.token_exists(token_id.clone())?;
        let uri = PreludeString::from_utf8(self.data::<Data>().token_uri.get(&token_id).unwrap()).unwrap();
        Ok(uri)
    }

    default fn get_token_royalty(&mut self, token_id: Id) -> Result<u16, PSP34Error> {
        self.token_exists(token_id.clone())?;
        let royalty = self.data::<Data>().royalty.get(&token_id).unwrap();
        Ok(royalty)
    }

    default fn get_royalty_info(&mut self, token_id: Id, price: Balance) -> Result<(Balance,AccountId),PSP34Error> {
        self.token_exists(token_id.clone())?;
        let royalty = u128::from(self.data::<Data>().royalty.get(&token_id).unwrap()) * price / 10000;
        let creator = self.data::<Data>().creator;
        Ok((royalty,creator))
    }
}

impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    default fn token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }

    default fn check_royalty(&self, royalty: u16) -> Result<(),PSP34Error>{
        if royalty > 5000 {
            return Err(PSP34Error::Custom(String::from(TokenError::OutOfBoundRoyaltyValue.as_str())))
        }
        Ok(())
    }
}