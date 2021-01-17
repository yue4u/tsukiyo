use crate::sql::schema::events;
use chrono::prelude::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use std::default::Default;
#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "An bar event")]
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
    pub publish_time: Option<NaiveDateTime>,
    pub page_view: Option<i32>,
    pub creator_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub published: bool,
    pub memo: Option<String>,
}

#[derive(Debug, Default, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[graphql(description = "An event input")]
#[table_name = "events"]
pub struct EventInput {
    pub slug: Option<String>,
    pub title: String,
    pub body: String,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub page_view: Option<i32>,
    pub creator_id: Option<i32>,
    pub published: Option<bool>,
    pub memo: Option<String>,
}

#[derive(Debug, Default, Insertable, AsChangeset, Serialize, Deserialize, GraphQLInputObject)]
#[table_name = "events"]
pub struct EventUpdate {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub publish_time: Option<NaiveDateTime>,
    pub page_view: Option<i32>,
    pub creator_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub published: Option<bool>,
    pub memo: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, GraphQLInputObject)]
pub struct EventQuery {
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub published: Option<bool>,
    pub limit: Option<i32>,
}

// pub fn seed() {
//     super::service::create("demo event", "demo event");
// }
