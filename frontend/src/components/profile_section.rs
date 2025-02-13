use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub name: String,
    pub description: String,
    pub about: String,
}

#[function_component]
pub fn ProfileSection(props: &ProfileProps) -> Html {
    html! {
        <div class="mb-12">
            <div class="mb-8">
                <h1 class="text-4xl font-extrabold text-gray-800 mb-2">{&props.name}</h1>
                <p class="text-lg text-gray-700 leading-relaxed">{&props.description}</p>
            </div>

            <div>
                <h2 class="text-2xl font-bold text-gray-800 mb-3">{ "About Me" }</h2>
                <p class="text-lg text-gray-500 leading-relaxed">{&props.about}</p>
            </div>
        </div>
    }
}
