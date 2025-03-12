use yew::prelude::*;

use crate::data::Experience;

#[derive(Properties, PartialEq)]
pub struct ExperienceSectionProps {
    pub experiences: Vec<Experience>,
    pub error: Option<String>,
}

#[function_component]
pub fn ExperienceSection(props: &ExperienceSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-20 mx-auto projects-section flex flex-col items-center"
            style="font-family: Courier; width: 80%;">
            <h1 class="text-2xl font-bold mb-3 text-center" style="color: #ADBCC6FF;">
                {"Work Experience"}
            </h1>

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

            <div class="space-y-4 flex flex-col items-center w-full">
                {props.experiences.iter().map(|exp| {
                    html! {
                        <div class="bg-black border border-gray-700 rounded-lg shadow-md p-6 transform transition-all duration-300 hover:shadow-xl hover:-translate-y-1 bg-gradient-to-br from-gray-900 via-black to-gray-800">
                            <div class="flex justify-between items-center mb-4">
                                <div class="flex items-center space-x-4 w-full overflow-hidden">
                                    <h3 class="text-xl font-semibold text-white whitespace-nowrap overflow-hidden text-ellipsis"
                                        style="color: #ADBCC6FF; font-style: italic; text-decoration: underline; letter-spacing: 0.05em; max-width: 50%;">
                                        {&exp.company}
                                    </h3>
                                    <p class="font-semibold whitespace-nowrap overflow-hidden text-ellipsis"
                                        style="color: #ADBCC6FF; max-width: 40%;">
                                        {&exp.position}
                                    </p>
                                </div>
                                <span class="text-sm text-gray-300 bg-gray-800 px-3 py-1 rounded-full flex-shrink-0 min-w-fit sm:w-auto"
                                    style="color: #718096;">
                                    {&exp.period}
                                </span>
                            </div>
                            <div class="mt-2 text-gray-300"
                                style="color:#718096;">
                                {&exp.description}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
