use yew::prelude::*;

use crate::data::Project;

#[derive(Properties, PartialEq)]
pub struct ProjectsSectionProps {
    pub projects: Vec<Project>,
    pub error: Option<String>,
}
#[function_component]
pub fn ProjectsSection(props: &ProjectsSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-4 mx-auto projects-section" style="font-family: Cursive; width: 80%;">
            <h1 class="text-2xl font-bold mb-3 text-center" style="color: #08504B;">{"Top Projects"}</h1>

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
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 justify-items-center mb-20">
                {
                    props.projects.iter().map(|project| {
                        html! {
                            <div key={project.id} class="border border-gray-700 rounded-lg transform hover:scale-105 transition-all flex flex-col bg-gradient-to-br from-gray-900 via-black to-gray-800">
                                <div class="px-6 py-4 flex-grow">
                                    <h3 class="text-base font-bold text-gray-800 mb-2 text-center"
                                        style="font-family: Cursive; color: #08504B;">
                                        {&project.name}
                                    </h3>
                                    <p class="text-sm mb-4 text-center"
                                       style="display: -webkit-box; -webkit-line-clamp: 5; color:#718096; -webkit-box-orient: vertical; overflow: hidden; text-overflow: ellipsis; font-family: Cursive;">
                                        {&project.description}
                                    </p>
                                </div>
                                <a
                                    href={project.link.clone()}
                                    class="inline-block px-4 py-2 text-sm text-white bg-blue-500 rounded-lg hover:bg-blue-600 transition mx-auto mb-4 relative overflow-hidden shimmer"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style="font-family: Cursive; position: relative; background: linear-gradient(90deg, #4F46E5 0%, #60A5FA 50%, #4F46E5 100%); background-size: 200% auto; animation: shimmer 2s infinite linear;"
                                >
                                    {"View Project"}
                                    <style>
                                        {"
                                        @keyframes shimmer {
                                            0% {
                                                background-position: 0% center;
                                            }
                                            100% {
                                                background-position: 200% center;
                                            }
                                        }
                                        "}
                                    </style>
                                </a>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
