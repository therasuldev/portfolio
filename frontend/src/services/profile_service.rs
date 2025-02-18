// // src/services/user_service.rs
// use gloo_net::http::Request;
// use serde::Deserialize;
// use yew::prelude::*;

// #[derive(Deserialize)]
// struct User {
//     fullname: String,
//     description: String,
//     about: String,
// }

// pub async fn fetch_profile_data(
//     name: UseStateHandle<String>,
//     description: UseStateHandle<String>,
//     about: UseStateHandle<String>,
//     error: UseStateHandle<Option<String>>,
// ) {
//     match Request::get("http://localhost:3000/profile").send().await {
//         Ok(response) => match response.json::<User>().await {
//             Ok(user) => {
//                 name.set(user.fullname);
//                 description.set(user.description);
//                 about.set(user.about);
//             }
//             Err(e) => {
//                 error.set(Some(format!("Failed to parse user data: {}", e)));
//             }
//         },
//         Err(e) => {
//             error.set(Some(format!("Failed to fetch user data: {}", e)));
//         }
//     }
// }

use crate::data::STATIC_PROFILE;
use yew::prelude::*;

pub async fn fetch_profile_data(
    name: UseStateHandle<String>,
    description: UseStateHandle<String>,
    about: UseStateHandle<String>,
    error: UseStateHandle<Option<String>>,
) {
    name.set(STATIC_PROFILE.name.clone());
    description.set(STATIC_PROFILE.description.clone());
    about.set(STATIC_PROFILE.about.clone());
}
