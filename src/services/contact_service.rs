use crate::data::{Contact, STATIC_CONTACTS};
use yew::prelude::*;

pub async fn fetch_contacts_data(
    contacts: UseStateHandle<Vec<Contact>>,
    _: UseStateHandle<Option<String>>, // handle error state
) {
    contacts.set(STATIC_CONTACTS.to_vec());
}
