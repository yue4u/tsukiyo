use crate::schema::events;
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
impl Event {
    fn dummy() -> Self {
        Self {
            id: 0,
            slug: Some("dummy slug".to_owned()),
            title: "title String".to_owned(),
            body: "body String".to_owned(),
            genre: Some("dummy genre".to_owned()),
            tag: Some("dummy tag".to_owned()),
            fee: Some(999),
            ogp_img: Some("dummy ogp_img".to_owned()),
            start_time: Some(chrono::Local::now().naive_utc()),
            end_time: Some(chrono::Local::now().naive_utc()),
            publish_time: Some(chrono::Local::now().naive_utc()),
            page_view: Some(999),
            creator_id: Some(0),
            created_at: chrono::Local::now().naive_utc(),
            updated_at: Some(chrono::Local::now().naive_utc()),
            published: true,
            memo: Some("dummy memo".to_owned()),
        }
    }
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

#[derive(Debug, Default, Insertable, AsChangeset, Serialize, Deserialize)]
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

// pub fn seed() {
//     super::service::create("demo event", "demo event");
// }
