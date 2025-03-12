use yew::prelude::*;

use crate::data::Contact;

#[derive(Properties, PartialEq)]
pub struct ContactsSectionProps {
    pub contacts: Vec<Contact>,
    pub error: Option<String>,
}

#[function_component]
pub fn ContactsSection(props: &ContactsSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-4 mx-auto projects-section px-4 md:px-6 lg:px-0" style="font-family: Courier; width: 90%;">
            <h1 class="text-2xl md:text-3xl font-bold mb-3 text-center" style="color: #ADBCC6FF;">{"Contact Me"}</h1>

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

            <div class="flex flex-wrap justify-center gap-4 md:gap-6 items-center">
                {
                    props.contacts.iter().map(|contact| {
                        let (icon_class, link) = match contact.platform.as_str() {
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
                                class="flex items-center space-x-3 md:space-x-4 p-3 md:p-4 border border-gray-700 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 ease-in-out transform hover:scale-105 hover:bg-gray-800 bg-gradient-to-br from-gray-900 via-black to-gray-800"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <i class={icon_class}></i>
                                <span class="text-base md:text-lg text-white font-medium hover:text-blue-400 transition-colors duration-200" style="font-size: 1.125rem; color: #718096; line-height: 1.6; letter-spacing: 0.05em;">
                                    {&contact.platform}
                                </span>
                            </a>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
