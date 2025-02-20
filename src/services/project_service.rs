use crate::data::{Project, STATIC_PROJECTS};
use yew::prelude::*;

pub async fn fetch_projects_data(
    projects: UseStateHandle<Vec<Project>>,
    _: UseStateHandle<Option<String>>, // handle error state
) {
    projects.set(STATIC_PROJECTS.to_vec());
}
