use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_project(conn: &PgConnection, name: &str, url_name: &str) -> Result<Project, Error> {
    let new_project = NewProject { name, url_name };
    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
}

pub fn insert_technology(
    conn: &PgConnection,
    name: &str,
    url_name: &str,
    image_url: &str,
) -> Result<Technology, Error> {
    let new_technology = NewTechnology {
        name,
        url_name,
        image_url,
    };
    diesel::insert_into(technologies::table)
        .values(&new_technology)
        .get_result(conn)
}

pub fn insert_adopt(
    conn: &PgConnection,
    projects_id: i64,
    technologies_id: i64,
    created_at: chrono::NaiveDateTime,
) -> Result<Adopt, Error> {
    let new_adopt = NewAdopt {
        projects_id,
        technologies_id,
        created_at,
    };
    diesel::insert_into(adopts::table)
        .values(&new_adopt)
        .get_result(conn)
}

fn get_technology_by_url_name(
    // 1
    conn: &PgConnection,
    tech_url_name: &str,
) -> Result<Technology, Error> {
    technologies::table
        .filter(technologies::url_name.eq(tech_url_name))
        .first(conn)
}

fn get_projects_by_technologies_id(
    // 2 & 3
    conn: &PgConnection,
    tech_id: i64,
) -> Result<Vec<Project>, Error> {
    projects::table
        .inner_join(adopts::table.on(projects::id.eq(adopts::projects_id)))
        .filter(adopts::technologies_id.eq(tech_id))
        .select(projects::all_columns)
        .load(conn)
}

pub fn get_technology_page_by_url_name(
    // 1 -> 2 & 3
    conn: &PgConnection,
    tech_url_name: &str,
) -> Result<(Technology, Vec<Project>), Error> {
    let tech = get_technology_by_url_name(&conn, tech_url_name)?;
    let projs = get_projects_by_technologies_id(&conn, tech.id)?;

    Ok((tech, projs))
}
