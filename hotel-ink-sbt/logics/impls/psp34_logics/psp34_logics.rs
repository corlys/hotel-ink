use crate::impls::psp34_logics::types::Data;
pub use crate::{
    impls::psp34_logics::types::{RoomType, RoomTypeIndex},
    traits::psp34_logics::Psp34Logics,
};

use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{enumerable::*, metadata::*},
    },
    modifiers,
    traits::{AccountId, Storage, String},
};

pub trait Internal {
    fn why_do_this(&self) -> Result<(), PSP34Error>;
}

impl<T> Psp34Logics for T
where
    T: Storage<Data>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    #[modifiers(only_owner)]
    default fn mint_token(
        &mut self,
        account: AccountId,
        id: Id,
        room_type: RoomTypeIndex,
    ) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(account, id.clone())?;
        self.data::<metadata::Data>()._set_attribute(
            id.clone(),
            String::from("room_type"),
            RoomType::to_openbrush_string(room_type),
        );
        self._emit_transfer_event(None, Some(account), id);
        Ok(())
    }
}

impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    default fn why_do_this(&self) -> Result<(), PSP34Error> {
        Ok(())
    }
}
