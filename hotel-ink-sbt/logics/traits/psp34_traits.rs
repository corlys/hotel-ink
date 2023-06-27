use ink::prelude::string::String as PreludeString;
use openbrush::contracts::traits::psp34::PSP34Error;

#[openbrush::wrapper]
pub type Psp34TraitRef = dyn Psp34Traits;

#[openbrush::trait_definition]
pub trait Psp34Traits {
    // add code here
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> PreludeString;

    #[ink(message)]
    fn set_base_uri(&mut self, base_uri: PreludeString) -> Result<(), PSP34Error>;
}
