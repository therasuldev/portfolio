use crate::{data::Language, data::STATIC_LANGUAGES};
use yew::prelude::*;

pub async fn fetch_language_data(
    languages: UseStateHandle<Vec<Language>>,
    _: UseStateHandle<Option<String>>, // handle error state
) {
    languages.set(STATIC_LANGUAGES.to_vec());
}
