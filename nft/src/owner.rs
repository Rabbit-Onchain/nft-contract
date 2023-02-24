use crate::*;

#[near_bindgen]
impl Contract {
    /// Get the owner of this contract
    pub fn get_owner(&self) -> AccountId {
        self.tokens.owner_id.clone()
    }

    /// Set the owner of this contract
    #[payable]
    pub fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_one_yocto();
        self.assert_owner();
        self.tokens.owner_id = owner_id;
    }
}
