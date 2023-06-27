#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod hotel_app {

    use ink::{
        codegen::{EmitEvent, Env},
        env::DefaultEnvironment,
        EnvAccess,
    };

    use openbrush::contracts::ownable::*;
    use openbrush::contracts::traits::psp34::Id;
    use openbrush::traits::Storage;

    use hotel_app_extension_pkg::{
        impls::hotel_app_logics::{
            hotel_app_logics::HotelEvents,
            types::{RoomType, RoomTypeIndex},
            *,
        },
        traits::hotel_app_traits::*,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct HotelAppContract {
        #[storage_field]
        ownable: ownable::Data,

        #[storage_field]
        hotel_app_logics: types::Data,
    }

    #[ink(event)]
    pub struct RoomCreated {
        room_id: Id,
        room_type: RoomTypeIndex,
    }

    #[ink(event)]
    pub struct RoomCheckedOut {
        room_id: Id,
        guest: AccountId,
    }

    #[ink(event)]
    pub struct SetRoomTypePrice {
        room_type: RoomTypeIndex,
        room_price: Balance,
    }

    impl Ownable for HotelAppContract {}

    impl HotelApp for HotelAppContract {}

    impl HotelAppContract {
        #[ink(constructor)]
        pub fn new(
            bronze_price: Balance,
            silver_price: Balance,
            gold_price: Balance,
            platinum_price: Balance,
            sbt_address: Option<AccountId>,
        ) -> Self {
            let mut instance = Self::default();
            instance
                .hotel_app_logics
                .type_to_price
                .insert(RoomType::Bronze.to_index(), &(bronze_price));
            instance
                .hotel_app_logics
                .type_to_price
                .insert(RoomType::Silver.to_index(), &(silver_price));
            instance
                .hotel_app_logics
                .type_to_price
                .insert(RoomType::Gold.to_index(), &(gold_price));
            instance
                .hotel_app_logics
                .type_to_price
                .insert(RoomType::Platinum.to_index(), &(platinum_price));
            instance.hotel_app_logics.sbt_address = sbt_address;
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }
    }

    impl HotelEvents for HotelAppContract {
        fn emit_room_created_event(&self, room_id: Id, room_type: RoomTypeIndex) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<HotelAppContract>>::emit_event::<
                RoomCreated,
            >(self.env(), RoomCreated { room_id, room_type });
        }
        fn emit_room_checks_out(&self, room_id: Id, guest: AccountId) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<HotelAppContract>>::emit_event::<
                RoomCheckedOut,
            >(self.env(), RoomCheckedOut { room_id, guest });
        }
        fn emit_set_type_price(&self, room_type: RoomTypeIndex, price: Balance) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<HotelAppContract>>::emit_event::<
                SetRoomTypePrice,
            >(
                self.env(),
                SetRoomTypePrice {
                    room_type,
                    room_price: price,
                },
            );
        }
    }
}
