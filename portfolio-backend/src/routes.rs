use axum::Json;
use crate::models::Project;

// Embed the data at compile time so the binary works from any working directory
// and never panics on a missing file at runtime.
const PROJECTS_JSON: &str = include_str!("data/projects.json");

pub async fn get_projects() -> Json<Vec<Project>> {
    let projects: Vec<Project> =
        serde_json::from_str(PROJECTS_JSON).expect("embedded projects.json is malformed");
    Json(projects)
}
