
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ToSchema, Validate)]
pub struct Content {
    #[schema(example = "youtube")]
    pub content_type: String,
    #[schema(example = "rust demo video link")]
    pub description: String,
    #[schema(example = "https://www.youtube.com/watch?v=aYsUBddY7KY&t=2s")]
    #[validate(url)]
    pub links: String,
    #[schema(example = true)]
    pub visibility: bool, // for public visibility value is true else it's value is false
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PubContent{
    pub id: String,
    pub username:String,
    pub content_type:String,
    pub description:String,
    pub links:String,
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ToSchema, Validate)]

pub struct User {
    #[schema(example = "Rammu")]
    #[validate(length(min = 3))]
    pub first_name: String,
    #[schema(example = "G")]
    pub last_name: String,
    #[schema(example = "RG")]
    #[validate(length(min = 2))]
    pub username: String,
    #[schema(example = "12345678")]
    #[validate(length(min = 4))]
    pub password: String  
}

// using Info struct to store sign username and password. 
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginCred {
    #[schema(example = "RG")]
    pub username: String,
    #[schema(example = "12345678")]
    pub password: String
}

// using Access struct to store which user is giving access to other user, for get access of private links of user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ToSchema)]
pub struct Access {
    #[schema(example = "RG")]
    pub my_username: String,
    #[schema(example = "RG")]
    pub friend_username: String
}
