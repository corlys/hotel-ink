use crate::impls::hotel_app_logics::types::{HotelError, RoomTypeIndex};

use openbrush::contracts::traits::psp34::Id;
use openbrush::traits::Balance;

#[openbrush::wrapper]
pub type HotelAppRef = dyn HotelApp;

#[openbrush::trait_definition]
pub trait HotelApp {
    //only_owner
    #[ink(message)]
    fn create_room(&mut self, room_id: Id, room_type: RoomTypeIndex) -> Result<(), HotelError>;

    //only_owner
    #[ink(message)]
    fn set_price_for_type(
        &mut self,
        room_type: RoomTypeIndex,
        room_price: Balance,
    ) -> Result<(), HotelError>;

    #[ink(message)]
    fn checks_out(&mut self, room_id: Id) -> Result<(), HotelError>;
}
