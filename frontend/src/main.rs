mod components;
mod services;

use components::{
    contacts_section::ContactsSection, profile_section::ProfileSection,
    projects_section::ProjectsSection, work_experience_section::WorkExperienceSection,
};
use services::{
    contact_service::fetch_contacts_data, profile_service::fetch_profile_data,
    project_service::fetch_projects_data, work_experience::fetch_work_experience_data,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
#[derive(Clone, PartialEq)]
enum ActiveSection {
    Profile,
    Projects,
    Experience,
    Contacts,
}
impl Default for ActiveSection {
    fn default() -> Self {
        ActiveSection::Profile
    }
}
#[function_component]
fn HomePage() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let about = use_state(String::new);
    let projects = use_state(Vec::new);
    let contacts = use_state(Vec::new);
    let experiences = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let active_section = use_state(|| ActiveSection::default());

    {
        let name = name.clone();
        let description = description.clone();
        let about = about.clone();
        let projects = projects.clone();
        let contacts = contacts.clone();
        let experiences = experiences.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                fetch_profile_data(name, description, about, error.clone()).await;
                fetch_projects_data(projects, error.clone()).await;
                fetch_contacts_data(contacts, error.clone()).await;
                fetch_work_experience_data(experiences, error.clone()).await;
            });
            || ()
        });
    };

    let icons = vec![
        (html! {<i class="fa fa-user"></i>}, ActiveSection::Profile),
        (html! {<i class="fa fa-code"></i>}, ActiveSection::Projects),
        (
            html! {<i class="fa fa-briefcase"></i>},
            ActiveSection::Experience,
        ),
        (
            html! {<i class="fa fa-envelope"></i>},
            ActiveSection::Contacts,
        ),
    ];

    let content = match *active_section {
        ActiveSection::Profile => {
            html! {
                <div class="flex flex-row min-h-screen">
                    <div class="w-1/2 p-4">
                        <img
                            src="https://bairesdev.mo.cloudinary.net/blog/2023/09/How-Many-Web-Developers-in-the-World-1.jpg?tx=w_1920,q_auto"
                            alt="Profile Picture"
                            class="w-full h-full object-cover"
                        />
                    </div>
                    <div class="w-2/5 p-4">
                        <ProfileSection
                            name={(*name).clone()}
                            description={(*description).clone()}
                            about={(*about).clone()}
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
                <WorkExperienceSection
                    experiences={(*experiences).clone()}
                />
            }
        }
        ActiveSection::Contacts => {
            html! {
                <ContactsSection
                    contacts={(*contacts).clone()}
                />
            }
        }
    };

    html! {
        <>
            <div class="flex min-h-screen bg-gradient-to-b from-blue-100 to-gray-200">
                <div class="flex-grow p-4">
                    {content}
                </div>
            </div>
            <div class="fixed top-0 right-0 h-screen w-1/10 p-4 bg-white bg-opacity-50 hover:bg-opacity-75 flex items-center justify-center">
                <nav class="flex flex-col items-center space-y-4">
                    { for icons.into_iter().map(|(icon, section)| {
                        let active_section = active_section.clone();
                        html! {
                            <button
                                class={format!(
                                    "text-2xl p-2 rounded-full {} {}",
                                    if *active_section == section {
                                        "text-indigo-600 scale-110"
                                    } else {
                                        "text-gray-600 scale-100"
                                    },
                                    if *active_section == section {
                                        "hover:text-indigo-800"
                                    } else {
                                        "hover:text-gray-800 hover:bg-gray-200 hover:bg-opacity-50"
                                    }
                                )}
                                onclick={move |_| active_section.set(section.clone())}
                            >
                                { icon }
                            </button>
                        }
                    })}
                </nav>
            </div>
        </>
    }
}

#[function_component]
fn NotFound() -> Html {
    html! {
        <div class="flex justify-center items-center h-screen">
            <h1 class="text-4xl font-bold text-gray-800">{ "404" }</h1>
            <p class="text-lg text-gray-600">{ "Page not found" }</p>
        </div>
    }
}
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
