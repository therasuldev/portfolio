use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactsSectionProps {
    pub contacts: Vec<Contact>,
    pub error: Option<String>,
}

#[function_component]
pub fn ContactsSection(props: &ContactsSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-4 mx-auto projects-section" style="font-family: 'Space Mono', monospace; width: 80%;">
            <h1 class="text-2xl font-bold mb-3 text-center" style="color: #08504B;">{"Contact Me"}</h1>

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

            <div class="flex justify-center gap-6 flex-wrap items-center">
                {
                    props.contacts.iter().map(|contact| {
                        let (icon_class, link) = match contact.name.as_str() {
                            "LinkedIn" => ("fab fa-linkedin text-4xl text-blue-400", contact.link.clone()),
                            "GitHub" => ("fab fa-github text-4xl text-gray-300", contact.link.clone()),
                            "Email" => ("fas fa-envelope text-4xl text-red-400", format!("mailto:{}", contact.link)),
                            "X" => ("fab fa-twitter text-4xl text-blue-400", contact.link.clone()),
                            "Youtube" => ("fab fa-youtube text-4xl text-red-700", contact.link.clone()),
                            _ => ("fas fa-globe text-4xl text-gray-400", contact.link.clone()),
                        };

                        html! {
                            <a
                                href={link}
                                class="flex items-center space-x-4 p-4 border border-gray-700 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 ease-in-out transform hover:scale-105 hover:bg-gray-800 bg-gradient-to-br from-gray-900 via-black to-gray-800"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <i class={icon_class}></i>
                                <span class="text-lg text-white font-medium hover:text-blue-400 transition-colors duration-200" style="font-size: 1.125rem; color: #718096; line-height: 1.6; letter-spacing: 0.05em;">
                                    {&contact.name}
                                </span>
                            </a>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
