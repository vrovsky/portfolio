use axum::Json;
use std::fs;
use crate::models::Project;

pub async fn get_projects() -> Json<Vec<Project>> {
    let data = fs::read_to_string("src/data/projects.json").expect("Unable to read file");
    let projects: Vec<Project> = serde_json::from_str(&data).expect("JSON was not well-formed");
    Json(projects)
}
