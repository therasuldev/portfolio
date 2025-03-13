use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use components::{
    contacts_section::ContactsSection, language_section::LanguageSection,
    profile_section::ProfileSection, projects_section::ProjectsSection,
    work_experience_section::ExperienceSection,
};
use services::{
    contact_service::fetch_contacts_data, language_service::fetch_language_data,
    profile_service::fetch_profile_data, project_service::fetch_projects_data,
    work_experience_service::fetch_work_experience_data,
};

use crate::{components, routes::AnimationState, services};

#[derive(Clone, PartialEq)]
pub enum ActiveSection {
    Profile,
    Projects,
    Experience,
    Language,
    Contacts,
}

impl Default for ActiveSection {
    fn default() -> Self {
        ActiveSection::Profile
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let about = use_state(String::new);
    let projects = use_state(Vec::new);
    let contacts = use_state(Vec::new);
    let experiences = use_state(Vec::new);
    let languages = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let active_section = use_state(|| ActiveSection::default());
    let animation_state = use_state(AnimationState::default);

    {
        let name = name.clone();
        let description = description.clone();
        let about = about.clone();
        let projects = projects.clone();
        let contacts = contacts.clone();
        let experiences = experiences.clone();
        let languages = languages.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                fetch_profile_data(name, description, about, error.clone()).await;
                fetch_projects_data(projects, error.clone()).await;
                fetch_contacts_data(contacts, error.clone()).await;
                fetch_language_data(languages, error.clone()).await;
                fetch_work_experience_data(experiences, error.clone()).await;
            });
            || ()
        });
    };

    // Extracted CSS styles
    let shake_animation = " 
    @keyframes shake {
        0%, 100% { transform: rotate(0deg); }
        25% { transform: rotate(-10deg); }
        50% { transform: rotate(10deg); }
        75% { transform: rotate(-5deg); }
    }
    .shake {
        animation: shake 0.5s ease-in-out;
    }
";

    // CSS classes as constants
    let main_container_class =
        "flex min-h-screen bg-gradient-to-br from-gray-900 via-black to-gray-800";
    let content_container_class = "flex-grow p-4 md:pr-24";

    let desktop_sidebar_class =
        "hidden md:fixed md:top-0 md:right-0 md:h-screen md:w-20 bg-white/20 backdrop-blur-sm \
                                md:flex items-center justify-center";
    let sidebar_nav_class = "flex flex-col items-center space-y-6";

    let mobile_navbar_class = "fixed bottom-0 left-0 w-full h-16 bg-white/20 backdrop-blur-sm \
                              flex md:hidden items-center justify-center";
    let mobile_nav_class = "flex flex-row items-center justify-around w-full px-4";

    let button_tooltip_container_class = "relative group";
    let tooltip_class =
        "absolute left-full ml-2 px-3 py-1 bg-black/75 text-white text-sm rounded-full \
                        opacity-0 group-hover:opacity-100 -translate-x-2 group-hover:translate-x-0 \
                        transition-all duration-300 pointer-events-none whitespace-nowrap";

    // Function to get button classes based on active state
    let get_button_classes = |is_active: bool, is_shaking: bool| {
        format!(
            "w-12 h-12 flex items-center justify-center rounded-full border-2 border-purple-600 \
            transition-all duration-300 {} {} {}",
            if is_active {
                "bg-purple-900 text-white shadow-lg scale-110 border-purple-600"
            } else {
                "text-white hover:bg-gray-800 hover:border-purple-400"
            },
            "transform hover:scale-110",
            if is_shaking { "shake" } else { "" }
        )
    };

    // Profile section CSS
    let profile_container_class =
        "flex flex-col md:flex-row items-center md:items-start min-h-screen p-4 md:space-x-8";
    let profile_image_container_class = "md:w-1/3 w-full flex justify-center";
    let profile_image_class = "w-48 h-48 md:w-96 md:h-96 object-cover rounded-lg";
    let profile_content_class = "flex flex-col space-y-4 md:w-2/3 w-full mt-4 md:mt-0";

    let icons = vec![
        (
            html! {<i class="fa fa-user"></i>},
            ActiveSection::Profile,
            "Profile",
        ),
        (
            html! {<i class="fa fa-code"></i>},
            ActiveSection::Projects,
            "Projects",
        ),
        (
            html! {<i class="fa fa-briefcase"></i>},
            ActiveSection::Experience,
            "Experience",
        ),
        (
            html! {<i class="fa fa-language"></i>},
            ActiveSection::Language,
            "Language",
        ),
        (
            html! {<i class="fa fa-envelope"></i>},
            ActiveSection::Contacts,
            "Contacts",
        ),
    ];

    let content = match *active_section {
        ActiveSection::Profile => {
            html! {
                <div class={profile_container_class}>
                    <div class={profile_image_container_class}>
                        <img
                            src="https://avatars.githubusercontent.com/u/74558294?v=4"
                            alt="Profile Picture"
                            class={profile_image_class}
                        />
                    </div>

                    <div class={profile_content_class}>
                        <ProfileSection
                            name={(*name).clone()}
                            description={(*description).clone()}
                            about={(*about).clone()}
                            error={(*error).clone()}
                        />
                    </div>
                </div>
            }
        }

        ActiveSection::Projects => {
            html! {
                <ProjectsSection
                    projects={(*projects).clone()}
                    error={(*error).clone()}
                />
            }
        }
        ActiveSection::Experience => {
            html! {
                <ExperienceSection
                    experiences={(*experiences).clone()}
                    error={(*error).clone()}
                />
            }
        }
        ActiveSection::Language => {
            html! {
                <LanguageSection
                    languages={(*languages).clone()}
                    error={(*error).clone()}
                />
            }
        }
        ActiveSection::Contacts => {
            html! {
                <ContactsSection
                    contacts={(*contacts).clone()}
                    error={(*error).clone()}
                />
            }
        }
    };

    html! {
        <>
            <style>
                { shake_animation }
            </style>
            <div class={main_container_class}>
                <div class={content_container_class}>
                    {content}
                </div>
            </div>

            // 🖥️ **DESKTOP SIDEBAR (Sağda qalan)**
            <div class={desktop_sidebar_class}>
                <nav class={sidebar_nav_class}>
                    { for icons.iter().map(|(icon, section, label)| {
                        let active_section = active_section.clone();
                        let animation_state = animation_state.clone();
                        let is_active = *active_section == *section;

                        let onclick = {
                            let section = section.clone();
                            let active_section = active_section.clone();
                            let animation_state = animation_state.clone();

                            Callback::from(move |_| {
                                if *active_section != section {
                                    animation_state.set(AnimationState {
                                        shaking: true,
                                        target_section: section.clone(),
                                    });

                                    let animation_state = animation_state.clone();
                                    let active_section = active_section.clone();
                                    let section = section.clone();

                                    gloo_timers::callback::Timeout::new(500, move || {
                                        active_section.set(section.clone());
                                        animation_state.set(AnimationState {
                                            shaking: false,
                                            target_section: section,
                                        });
                                    })
                                    .forget();
                                }
                            })
                        };

                        let is_shaking = animation_state.shaking && animation_state.target_section == *section;
                        html! {
                            <div class={button_tooltip_container_class}>
                                <button
                                    class={get_button_classes(is_active, is_shaking)}
                                    onclick={onclick}
                                >
                                    <div class="text-xl">
                                        { icon.clone() }
                                    </div>
                                </button>
                                <div class={tooltip_class}>
                                    { label }
                                </div>
                            </div>
                        }
                    })}
                </nav>
            </div>

            // 📱 **MOBILE BOTTOM NAVBAR (Ekranın aşağısında)**
            <div class={mobile_navbar_class}>
                <nav class={mobile_nav_class}>
                    { for icons.iter().map(|(icon, section, _label)| {
                        let active_section = active_section.clone();
                        let animation_state = animation_state.clone();
                        let is_active = *active_section == *section;

                        let onclick = {
                            let section = section.clone();
                            let active_section = active_section.clone();
                            let animation_state = animation_state.clone();

                            Callback::from(move |_| {
                                if *active_section != section {
                                    animation_state.set(AnimationState {
                                        shaking: true,
                                        target_section: section.clone(),
                                    });

                                    let animation_state = animation_state.clone();
                                    let active_section = active_section.clone();
                                    let section = section.clone();

                                    gloo_timers::callback::Timeout::new(500, move || {
                                        active_section.set(section.clone());
                                        animation_state.set(AnimationState {
                                            shaking: false,
                                            target_section: section,
                                        });
                                    })
                                    .forget();
                                }
                            })
                        };

                        let is_shaking = animation_state.shaking && animation_state.target_section == *section;
                        html! {
                            <button
                                class={get_button_classes(is_active, is_shaking)}
                                onclick={onclick}
                            >
                                <div class="text-xl">
                                    { icon.clone() }
                                </div>
                            </button>
                        }
                    })}
                </nav>
            </div>
        </>
    }
}
