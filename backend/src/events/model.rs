use crate::sql::schema::events;
use crate::utils::not_empty;
use chrono::prelude::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use std::default::Default;
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Event data including admin info")]
pub struct Event {
    pub id: i32,
    pub slug: Option<String>,
    pub title: String,
    pub body: String,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub publish_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub page_view: i32,
    pub creator_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub published: bool,
    pub memo: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Event data which is ok to be public")]
pub struct EventPublic {
    pub id: i32,
    pub slug: Option<String>,
    pub title: String,
    pub body: String,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub publish_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub page_view: i32,
}

#[derive(Debug, Default, Validate, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[graphql(description = "Event input for creating event")]
#[table_name = "events"]
pub struct EventInput {
    #[validate(custom = "not_empty")]
    pub slug: Option<String>,
    #[validate(custom = "not_empty")]
    pub title: String,
    #[validate(custom = "not_empty")]
    pub body: String,
    #[validate(custom = "not_empty")]
    pub genre: Option<String>,
    #[validate(custom = "not_empty")]
    pub tag: Option<String>,
    pub fee: Option<i32>,
    #[validate(custom = "not_empty")]
    pub ogp_img: Option<String>,
    pub end_time: Option<NaiveDateTime>,
    pub creator_id: Option<i32>,
    pub published: Option<bool>,
    #[validate(custom = "not_empty")]
    pub memo: Option<String>,
}

#[derive(
    Debug, Default, Validate, Insertable, AsChangeset, Serialize, Deserialize, GraphQLInputObject,
)]
#[table_name = "events"]
pub struct EventUpdate {
    #[validate(custom = "not_empty")]
    pub slug: Option<String>,
    #[validate(custom = "not_empty")]
    pub title: Option<String>,
    #[validate(custom = "not_empty")]
    pub body: Option<String>,
    #[validate(custom = "not_empty")]
    pub genre: Option<String>,
    #[validate(custom = "not_empty")]
    pub tag: Option<String>,
    pub fee: Option<i32>,
    #[validate(custom = "not_empty")]
    pub ogp_img: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub publish_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub page_view: i32,
    pub creator_id: Option<i32>,
    pub published: Option<bool>,
    #[validate(custom = "not_empty")]
    pub memo: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, GraphQLInputObject)]
pub struct EventQuery {
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub published: Option<bool>,
    pub limit: Option<i32>,
}
