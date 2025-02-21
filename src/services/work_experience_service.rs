use crate::data::{Experience, STATIC_EXPERIENCES};
use yew::prelude::*;

pub async fn fetch_work_experience_data(
    experiences: UseStateHandle<Vec<Experience>>,
    _: UseStateHandle<Option<String>>, // handle error state
) {
    experiences.set(STATIC_EXPERIENCES.to_vec());
}
