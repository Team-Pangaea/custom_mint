#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod token {
    
    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::ownable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;

	use ink_storage::traits::SpreadAllocate;

	use custom_mint_pkg::{
		traits::custom_mint::*,
		impls::custom_mint::*,
	};

    #[ink(storage)]
    #[derive(Default, Storage,SpreadAllocate)]
    pub struct Token {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
		#[storage_field]
		custom_mint: types::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP34 for Token {}
	impl Ownable for Token {}
	impl PSP34Enumerable for Token {}
	impl PSP34Metadata for Token {}
	impl CustomMint for Token {}
     
    impl Token {
        #[ink(constructor)]
		pub fn new(
			name: String,
			symbol: String,
			base_uri: String,
		) -> Self {
			ink_lang::codegen::initialize_contract(|instance: &mut Token|{
				instance._init_with_owner(instance.env().caller());
				let collection_id = instance.collection_id();
				instance._set_attribute(collection_id.clone(), String::from("name"), name);
				instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
				instance._set_attribute(collection_id, String::from("baseUri"), base_uri);
				instance.custom_mint.last_token_id = 0;
				instance.custom_mint.creator = instance.env().caller();
			})
		}

    }

	#[cfg(test)]
	mod tests {
		use super::*;
        use crate::token::PSP34Error::*;
        use ink_env::test;
        use ink_lang as ink;
		use ink_prelude::string::String as PreludeString;

		#[ink::test]
		fn new_works() {
			let contract = Token::new(String::from("Test"),String::from("TST"),String::from("https://ipfs/1"));
			let collection_id = contract.collection_id();

			assert_eq!(contract.get_attribute(collection_id.clone(), String::from("name")),
			Some(String::from("Test")));

			assert_eq!(contract.get_attribute(collection_id.clone(), String::from("symbol")),
			Some(String::from("TST")));

			assert_eq!(contract.get_attribute(collection_id.clone(), String::from("baseUri")),
			Some(String::from("https://ipfs/1")));
		}

		#[ink::test]
		fn mint_works() {
			let mut contract = Token::new(String::from("Test"),String::from("TST"),String::from("https://ipfs/1"));
			let collection_id = contract.collection_id();

			let accounts = default_accounts();

			assert_eq!(contract.owner(),accounts.alice);

			set_sender(accounts.bob);

			assert_eq!(contract.total_supply(), 0);

			let token_uri = String::from("Token1");
			let royalty = 100;// 1%
            let sales_price = 100;
            let royalties = 1;

			assert!(contract.mint(accounts.bob,token_uri,royalty).is_ok());

			assert_eq!(contract.total_supply(), 1);

			assert_eq!(contract.owner_of(Id::U64(1)), Some(accounts.bob));

            assert_eq!(contract.balance_of(accounts.bob), 1);

			assert_eq!(contract.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(contract.custom_mint.last_token_id, 1);
			assert_eq!(contract.get_token_uri(Id::U64(1)),Ok(PreludeString::from("Token1")));
			assert_eq!(contract.get_token_royalty(Id::U64(1)),Ok(royalty));
			assert_eq!(contract.get_royalty_info(Id::U64(1),sales_price),Ok((royalties,accounts.alice)));
		}

		fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }
	}
}