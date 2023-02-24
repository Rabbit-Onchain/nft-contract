use crate::*;

impl Contract {
    /// only operator if is an owner of an operator
    pub fn assert_owner(&self) {
        assert!(
            env::predecessor_account_id() == self.tokens.owner_id,
            "Can only be called by the owner"
        );
    }

    /// Assert that 1 yoctoNEAR was attached
    pub fn assert_one_yocto(&self) {
        assert_eq!(
            env::attached_deposit(),
            1,
            "Requires attached deposit of exactly 1 yoctoNEAR"
        )
    }
}
