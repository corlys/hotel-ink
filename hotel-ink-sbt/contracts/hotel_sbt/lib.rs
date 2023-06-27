#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::hotel_sbt::{HotelSbtContract, HotelSbtContractRef};

#[openbrush::contract]
pub mod hotel_sbt {

    use ink::codegen::{EmitEvent, Env};
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::{
            extensions::{enumerable::*, metadata::*},
            Transfer as TransferImpl,
        },
        traits::{Storage, String},
    };

    use hotel_sbt_extension_pkg::{
        impls::psp34_logics::*, traits::psp34_logics::*, traits::psp34_traits::*,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct HotelSbtContract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,

        #[storage_field]
        ownable: ownable::Data,

        #[storage_field]
        metadata: metadata::Data,

        #[storage_field]
        psp34_logics: types::Data,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    impl PSP34 for HotelSbtContract {}
    impl PSP34Enumerable for HotelSbtContract {}
    impl Ownable for HotelSbtContract {}

    impl Psp34Traits for HotelSbtContract {}
    impl Psp34Logics for HotelSbtContract {}

    impl TransferImpl for HotelSbtContract {
        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _token_id: &Id,
        ) -> Result<(), PSP34Error> {
            let address_zero: AccountId = [0u8; 32].into();
            //if not owner cannot send to anyone
            if let Some(address) = from {
                if address != &address_zero {
                    if address == &self.owner() {
                        Ok(())
                    } else {
                        return Err(PSP34Error::Custom(String::from(
                            "Only owner can transfer this token",
                        )));
                    }
                } else {
                    return Err(PSP34Error::Custom(String::from("address is zero")));
                }
            } else {
                return Err(PSP34Error::Custom(String::from("address is none")));
            }
        }
    }

    // Override event emission methods
    impl psp34::Internal for HotelSbtContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }

    impl HotelSbtContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(instance.env().caller());
            let collection_id = instance.collection_id();
            instance._set_attribute(
                collection_id.clone(),
                String::from("name"),
                String::from("default_name"),
            );
            instance._set_attribute(
                collection_id.clone(),
                String::from("symbol"),
                String::from("default_symbol"),
            );
            instance._set_attribute(
                collection_id.clone(),
                String::from("base_uri"),
                String::from("default_base_uri"),
            );
            instance
        }
    }
}
