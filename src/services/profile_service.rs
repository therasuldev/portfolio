use crate::data::STATIC_PROFILE;
use yew::prelude::*;

pub async fn fetch_profile_data(
    name: UseStateHandle<String>,
    description: UseStateHandle<String>,
    about: UseStateHandle<String>,
    _: UseStateHandle<Option<String>>, // handle error state
) {
    name.set(STATIC_PROFILE.name.clone());
    description.set(STATIC_PROFILE.description.clone());
    about.set(STATIC_PROFILE.about.clone());
}
