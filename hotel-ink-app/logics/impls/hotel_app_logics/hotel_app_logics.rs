use crate::{
    impls::hotel_app_logics::types::{Data, HotelError, RoomType, RoomTypeIndex},
    traits::hotel_app_traits::HotelApp,
};

use hotel_sbt::HotelSbtContractRef;

use ink::codegen::Env;
use ink::prelude::vec::Vec;

use openbrush::{
    contracts::{
        ownable::*,
        psp34::{PSP34Ref, *},
    },
    modifiers,
    traits::{AccountId, Balance, Storage},
};

pub trait Internal {
    fn check_type_price(&self) -> Balance;

    fn check_loyalty(&self) -> u32;
}

pub trait HotelEvents {
    fn emit_room_created_event(&self, room_id: Id, room_type: RoomTypeIndex);
    fn emit_room_checks_out(&self, room_id: Id, guest: AccountId);
    fn emit_set_type_price(&self, room_type: RoomTypeIndex, price: Balance);
}

impl<T> HotelApp for T
where
    T: Storage<Data> + Storage<ownable::Data>,
{
    #[modifiers(only_owner)]
    default fn create_room(
        &mut self,
        room_id: Id,
        room_type: RoomTypeIndex,
    ) -> Result<(), HotelError> {
        // 0. checks if mapping of room_to_type already exist (DONE)
        // 1. create a mapping to a storage room_to_type (DONE)
        // 2. mint the sbt to the contract
        // 2. emit the room creation event (DONE)
        let mapping_if_exists = self.data::<Data>().room_to_type.get(&room_id).unwrap_or(0);
        if RoomType::from(mapping_if_exists) == RoomType::Empty {
            return Err(HotelError::RoomIsAlreadyCreated);
        }
        self.data::<Data>()
            .room_to_type
            .insert(room_id.clone(), &room_type);
        let sbt_address = self.data::<Data>().sbt_address.unwrap();
        let current_contract_id = Self::env().account_id();
        match HotelSbtContractRef::mint_token(
            &sbt_address,
            current_contract_id,
            room_id.clone(),
            room_type.clone(),
        ) {
            Ok(()) => {
                self.data::<Data>()
                    .emit_room_created_event(room_id, room_type);
                Ok(())
            }
            Err(_) => Err(HotelError::ErrorMintSBT),
        }
    }

    #[modifiers(only_owner)]
    default fn set_price_for_type(
        &mut self,
        room_type: RoomTypeIndex,
        room_price: Balance,
    ) -> Result<(), HotelError> {
        // 0. check if the RoomTypeIndex is the correct room type (available and not empty)
        // 1. insert the price to the type_to_price mapping
        let room_type_index = RoomType::from(room_type.clone());
        if room_type_index == RoomType::Empty {
            return Err(HotelError::CannotChangePriceRoomEmpty);
        }
        self.data::<Data>()
            .type_to_price
            .insert(room_type.clone(), &(room_price));
        self.data::<Data>()
            .emit_set_type_price(room_type, room_price);
        Ok(())
    }

    default fn checks_out(&mut self, room_id: Id) -> Result<(), HotelError> {
        // 0. check the ownership and approval of the nft, make sure this contract has approval
        // 1. sends the nft to address 0 / burn it
        // 3. emit Event
        let sbt_address = self.data::<Data>().sbt_address.unwrap();
        let caller = Self::env().caller();
        let current_contract_id = Self::env().account_id();
        match PSP34Ref::allowance(
            &sbt_address,
            caller.clone(),
            current_contract_id,
            Some(room_id.clone()),
        ) {
            true => match PSP34Ref::transfer(
                &sbt_address,
                [0u8; 32].into(),
                room_id.clone(),
                Vec::new(),
            ) {
                Ok(()) => {
                    self.data::<Data>().emit_room_checks_out(room_id, caller);
                    Ok(())
                }
                Err(error) => Err(HotelError::PSP34Error(error)),
            },
            false => Err(HotelError::TokenNotApproved),
        }
    }
}

impl<T> HotelEvents for T
where
    T: Storage<Data>,
{
    default fn emit_room_created_event(&self, _room_id: Id, _room_type: RoomTypeIndex) {}
    default fn emit_room_checks_out(&self, _room_id: Id, _guest: AccountId) {}
    default fn emit_set_type_price(&self, _room_type: RoomTypeIndex, _price: Balance) {}
}
