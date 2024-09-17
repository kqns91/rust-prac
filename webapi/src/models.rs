use super::schema::*;

// projectsテーブル用
#[derive(Qeryable)] // SQL で取得された値を読み込むために使用
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
