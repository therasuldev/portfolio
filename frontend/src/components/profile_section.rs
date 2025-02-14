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
        <div style="font-family: 'Space Mono', monospace; margin-bottom: 3rem; width: 80%;">
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
            <div style="margin-bottom: 2rem;">
                <h1 style="font-size: 2.25rem; font-weight: 800; color: #08504BFF; margin-bottom: 0.5rem; letter-spacing: 0.05em;">
                    { &props.name }
                </h1>
                <p style="font-size: 1.125rem; color: #718096; line-height: 1.6; letter-spacing: 0.05em;">
                    { &props.description }
                </p>
            </div>

            <div>
                <p style="font-size: 1.125rem; color: #718096; line-height: 1.6; letter-spacing: 0.05em;">
                    { &props.about }
                </p>
            </div>
        </div>
    }
}
