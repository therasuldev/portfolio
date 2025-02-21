use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub name: String,
    pub description: String,
    pub about: String,
    pub error: Option<String>,
}

#[function_component]
pub fn ProfileSection(props: &ProfileProps) -> Html {
    html! {
        <div class="w-full md:w-4/5 mb-12" style="font-family: 'Space Mono', monospace;">
        {
            if let Some(error_message) = &props.error {
                html! {
                    <div class="w-full bg-red-50 border-l-4 border-red-400 text-red-700 p-4 mb-6 rounded-md">
                        <p class="font-medium text-center">{error_message}</p>
                    </div>
                }
            } else {
                html! {}
            }
        }
            <div class="mb-8 text-center md:text-left" style="font-family: Cursive;">
                <h1 class="text-4xl font-extrabold mb-2 tracking-wide"
                    style="color: #08504B;">
                    { &props.name }
                </h1>
                <p class="text-lg text-gray-600 leading-relaxed tracking-wide">
                    { &props.description }
                </p>

                <p class="text-lg text-gray-600 leading-relaxed tracking-wide mt-4">
                    { &props.about }
                </p>

            </div>


        </div>
    }
}
