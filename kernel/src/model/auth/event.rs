use crate::id::UserId;

pub struct CreateToken {
    pub user_id: UserId,
    pub access_token: String,
}

impl CreateToken {
    pub fn new(user_id: UserId) -> Self {
        let access_token = UserId::new().into();

        Self {
            user_id,
            access_token,
        }
    }
}
