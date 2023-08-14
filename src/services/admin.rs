use mongodb::bson::{oid::ObjectId, doc};

use crate::{
    dtos::admin::CreateUser, errors::api_errors::{APIResult, APIError}, models::user_model::UserModel, database::mongo::MongoDB,
};

#[derive(Clone)]
pub struct AdminService {
    pub user_collection: mongodb::Collection<UserModel>,
}

impl AdminService {
    pub async fn new() -> Self {

        Self {
            user_collection: MongoDB::new().await.connection.collection("users"),
        }
    }
}

impl AdminService {
    fn hash_password(password: String) -> APIResult<String> {
        Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
    }

    pub async fn create_hr_profile(&self, create_user: CreateUser) -> APIResult<()> {
        let user: Option<UserModel> = self.user_collection.find_one(doc!{"email": &create_user.email}, None).await?;

        if user.is_some() {
            return Err(APIError::UserAlreadyExists);
        }

        let user = UserModel {
            _id: Some(ObjectId::new()),
            email: Some(create_user.email.clone()),
            password: Some(AdminService::hash_password(create_user.password)?),
            roles: create_user.roles,
            ..Default::default()
        };

        self.user_collection.insert_one(user, None).await?;

        Ok(())
    }
}
