use ink::storage::Mapping;
use openbrush::traits::{Balance, String};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(PartialEq)]
pub enum RoomType {
    Empty,
    Bronnze,
    Silver,
    Gold,
    Platinum,
}

pub type RoomTypeIndex = u8;

impl RoomType {
    pub fn to_index(&self) -> u8 {
        match self {
            RoomType::Empty => 0,
            RoomType::Bronnze => 1,
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
            RoomType::Bronnze
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
    pub room_to_type: Mapping<u32, RoomTypeIndex>,
    pub type_to_price: Mapping<RoomTypeIndex, Balance>,
}

pub enum SBTError {
    CannotBeTransferred,
}

impl SBTError {
    pub fn as_str(&self) -> String {
        match self {
            SBTError::CannotBeTransferred => String::from("SBT Token cannot be transferred"),
        }
    }
}
