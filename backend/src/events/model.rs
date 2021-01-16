use crate::schema::events;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::time::SystemTime;
// use serde_json::Result;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub slug: Option<String>,
    pub title: String,
    pub body: String,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub publish_time: Option<SystemTime>,
    pub page_view: Option<i32>,
    pub creator_id: Option<i32>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub published: bool,
    pub memo: Option<String>,
}

#[derive(Debug, Default, Insertable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub slug: Option<String>,
    pub title: String,
    pub body: String,
    pub genre: Option<String>,
    pub tag: Option<String>,
    pub fee: Option<i32>,
    pub ogp_img: Option<String>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub publish_time: Option<SystemTime>,
    pub page_view: Option<i32>,
    pub creator_id: Option<i32>,
    pub updated_at: Option<SystemTime>,
    pub memo: Option<String>,
}

// pub fn seed() {
//     super::service::create("demo event", "demo event");
// }
