use crate::sql::schema::contacts;
use crate::utils::is_not_empty;
use chrono::prelude::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use std::default::Default;
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "A new contact")]
pub struct Contact {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub checked: bool,
}

#[derive(Debug, Default, Validate, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[graphql(description = "A new contact input")]
#[table_name = "contacts"]
pub struct ContactInput {
    #[validate(length(max = 100), custom = "is_not_empty")]
    pub title: String,
    #[validate(length(max = 50), custom = "is_not_empty")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub phone: Option<String>,
    #[validate(custom = "is_not_empty")]
    pub body: String,
}

#[derive(Debug, Default, Insertable, AsChangeset, Serialize, Deserialize, GraphQLInputObject)]
#[table_name = "contacts"]
pub struct ContactUpdate {
    checked: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, GraphQLInputObject)]
pub struct ContactQuery {
    pub search_string: Option<String>,
    pub checked: Option<bool>,
}
