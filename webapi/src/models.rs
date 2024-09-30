use super::schema::*;
use diesel::{Insertable, Queryable}; // サンプルでは不要そう。diesel が v2.2.4 の影響?
use serde::{Deserialize, Serialize};

// projectsテーブル用
#[derive(Queryable, Serialize, Deserialize)] // SQL で取得された値を読み込むために使用
pub struct Project {
    pub id: i64,
    pub name: String,
    pub url_name: String,
}

#[derive(Insertable)] // SQL で挿入するために使用
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
}

// technologiesテーブル用
#[derive(Queryable, Serialize, Deserialize)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub url_name: String,
    pub image_url: String,
}

#[derive(Insertable)]
#[table_name = "technologies"]
pub struct NewTechnology<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub image_url: &'a str,
}

// adoptsテーブル用
#[derive(Queryable)]
pub struct Adopt {
    pub id: i64,
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "adopts"]
pub struct NewAdopt {
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}
