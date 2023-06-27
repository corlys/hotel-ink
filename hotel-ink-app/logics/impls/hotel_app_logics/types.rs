use ink::storage::Mapping;
use openbrush::contracts::traits::{
    ownable::OwnableError,
    psp34::{Id, PSP34Error},
};
use openbrush::traits::{AccountId, Balance, String};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(PartialEq)]
pub enum RoomType {
    Empty,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

pub type RoomTypeIndex = u8;

impl RoomType {
    pub fn to_index(&self) -> u8 {
        match self {
            RoomType::Empty => 0,
            RoomType::Bronze => 1,
            RoomType::Silver => 2,
            RoomType::Gold => 3,
            RoomType::Platinum => 4,
        }
    }

    pub fn from(index: u8) -> Self {
        if index == 4 {
            RoomType::Platinum
        } else if index == 3 {
            RoomType::Gold
        } else if index == 2 {
            RoomType::Silver
        } else if index == 1 {
            RoomType::Bronze
        } else {
            RoomType::Empty
        }
    }

    pub fn to_openbrush_string(index: u8) -> String {
        if index == 4 {
            String::from("Platinum")
        } else if index == 3 {
            String::from("Gold")
        } else if index == 2 {
            String::from("Silver")
        } else if index == 1 {
            String::from("Bronze")
        } else {
            String::from("Empty")
        }
    }
}

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub room_to_type: Mapping<Id, RoomTypeIndex>,
    pub type_to_price: Mapping<RoomTypeIndex, Balance>,
    pub guest_loyalty_counter: Mapping<Option<AccountId>, u32>,
    pub sbt_address: Option<AccountId>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum HotelError {
    SBTContractIsNotSet,
    ErrorMintSBT,
    TokenNotApproved,
    FundIsNotEnough,
    RoomIeOccupied,
    RoomIsAlreadyCreated,
    CannotChangePriceRoomEmpty,
    OwnableError(OwnableError),
    PSP34Error(PSP34Error),
}

impl From<OwnableError> for HotelError {
    fn from(error: OwnableError) -> Self {
        HotelError::OwnableError(error)
    }
}

impl From<PSP34Error> for HotelError {
    fn from(error: PSP34Error) -> Self {
        HotelError::PSP34Error(error)
    }
}
