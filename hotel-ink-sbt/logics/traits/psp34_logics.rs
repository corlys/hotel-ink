use openbrush::{
    contracts::psp34::{extensions::metadata::*, PSP34Error},
    traits::AccountId,
};

use crate::impls::psp34_logics::types::RoomTypeIndex;

#[openbrush::wrapper]
pub type Psp34LogicsRef = dyn Psp34Logics;

#[openbrush::trait_definition]
pub trait Psp34Logics {
    // add code here
    #[ink(message)]
    fn mint_token(
        &mut self,
        account: AccountId,
        id: Id,
        room_type: RoomTypeIndex,
    ) -> Result<(), PSP34Error>;
}
