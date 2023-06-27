use crate::impls::psp34_logics::types::Data;
use ink::prelude::string::{String as PreludeString, ToString};
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{enumerable::*, metadata::*},
    },
    modifiers,
    traits::{Storage, String},
};

pub use crate::traits::psp34_traits::Psp34Traits;

pub trait Internal {
    fn token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}

impl<T> Psp34Traits for T
where
    T: Storage<Data>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    default fn token_uri(&self, token_uri: u64) -> PreludeString {
        let base_uri = self.get_attribute(
            self.data::<psp34::Data<enumerable::Balances>>()
                .collection_id(),
            String::from("baseUri"),
        );
        let mut uri = PreludeString::from_utf8(base_uri.unwrap()).unwrap();
        uri = uri + &token_uri.to_string() + &PreludeString::from(".json");
        uri
    }

    #[modifiers[only_owner]]
    default fn set_base_uri(&mut self, base_uri: PreludeString) -> Result<(), PSP34Error> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()._set_attribute(
            id,
            String::from("baseUri"),
            base_uri.into_bytes(),
        );
        Ok(())
    }
}

impl<T> Internal for T
where
    T: Storage<psp34::Data<enumerable::Balances>>,
{
    default fn token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            ._owner_of(&id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}
