use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::routes::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {
        <div class="flex flex-col justify-center items-center min-h-screen bg-gradient-to-br from-gray-900 via-black to-gray-800 text-white p-4">
            <div class="text-9xl font-bold text-purple-500 mb-4">{ "404" }</div>
            <h1 class="text-4xl font-bold mb-6">{ "Page Not Found" }</h1>
            <p class="text-xl text-gray-300 mb-8 text-center max-w-md">
                { "The page you are looking for might have been removed or is temporarily unavailable." }
            </p>
            <button
                onclick={go_home}
                class="px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-full text-white font-medium transition-all duration-300 flex items-center"
            >
                <i class="fa fa-home mr-2"></i>
                { "Back to Home" }
            </button>
        </div>
    }
}
