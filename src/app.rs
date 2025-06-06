use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::routes::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
