use yew::prelude::*;

use crate::data::{Language, LanguageSkills};

#[derive(Properties, PartialEq)]
pub struct LanguageSectionProps {
    pub languages: Vec<Language>,
    pub error: Option<String>,
}

#[function_component]
pub fn LanguageSection(props: &LanguageSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-8 mx-auto px-4 md:px-6 lg:px-0" style="font-family: Cursive; width: 90%;">
        <h1 class="text-2xl md:text-3xl font-bold mb-3 text-center " style="color: #08504B;">{"Language Skills"}</h1>

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

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                {
                    props.languages.iter().map(|lang| {
                        html! {
                            <div class="bg-black border border-gray-700 rounded-lg shadow-md p-6 transform transition-all duration-300 hover:shadow-xl hover:-translate-y-1 bg-gradient-to-br from-gray-900 via-black to-gray-800">
                                <div class="flex items-center justify-between mb-6">
                                    <div class="flex items-center space-x-4">
                                        <span class="text-3xl">{&lang.flag_emoji}</span>
                                        <div>
                                            <h3 class="text-xl font-bold text-gray-200">{&lang.name}</h3>
                                            <p class="text-gray-400 text-sm">{&lang.native_name}</p>
                                        </div>
                                    </div>
                                    {
                                        if let Some(cert) = &lang.certification {
                                            html! {
                                                <div class="px-3 py-1 bg-blue-500/20 text-blue-400 rounded-full text-sm">
                                                    {cert}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>

                                <div class="space-y-4">
                                    // Speaking
                                    <div>
                                        <div class="flex justify-between mb-1">
                                            <span class="text-gray-300">{"Speaking"}</span>
                                            <span class="text-blue-400">{format!("{}%", lang.skills.speaking)}</span>
                                        </div>
                                        <div class="w-full bg-gray-700 rounded-full h-2">
                                            <div
                                                class="bg-gradient-to-r from-blue-600 to-blue-400 h-2 rounded-full transition-all duration-500 ease-out"
                                                style={format!("width: {}%", lang.skills.speaking)}
                                            />
                                        </div>
                                    </div>

                                    // Writing
                                    <div>
                                        <div class="flex justify-between mb-1">
                                            <span class="text-gray-300">{"Writing"}</span>
                                            <span class="text-green-400">{format!("{}%", lang.skills.writing)}</span>
                                        </div>
                                        <div class="w-full bg-gray-700 rounded-full h-2">
                                            <div
                                                class="bg-gradient-to-r from-green-600 to-green-400 h-2 rounded-full transition-all duration-500 ease-out"
                                                style={format!("width: {}%", lang.skills.writing)}
                                            />
                                        </div>
                                    </div>

                                    // Reading
                                    <div>
                                        <div class="flex justify-between mb-1">
                                            <span class="text-gray-300">{"Reading"}</span>
                                            <span class="text-purple-400">{format!("{}%", lang.skills.reading)}</span>
                                        </div>
                                        <div class="w-full bg-gray-700 rounded-full h-2">
                                            <div
                                                class="bg-gradient-to-r from-purple-600 to-purple-400 h-2 rounded-full transition-all duration-500 ease-out"
                                                style={format!("width: {}%", lang.skills.reading)}
                                            />
                                        </div>
                                    </div>

                                    // Listening
                                    <div>
                                        <div class="flex justify-between mb-1">
                                            <span class="text-gray-300">{"Listening"}</span>
                                            <span class="text-yellow-400">{format!("{}%", lang.skills.listening)}</span>
                                        </div>
                                        <div class="w-full bg-gray-700 rounded-full h-2">
                                            <div
                                                class="bg-gradient-to-r from-yellow-600 to-yellow-400 h-2 rounded-full transition-all duration-500 ease-out"
                                                style={format!("width: {}%", lang.skills.listening)}
                                            />
                                        </div>
                                    </div>
                                </div>

                                <div class="mt-4 pt-4 border-t border-gray-700/50">
                                    <p class="text-gray-400 text-sm">
                                        {get_overall_level_description(calculate_overall_level(&lang.skills))}
                                    </p>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

fn calculate_overall_level(skills: &LanguageSkills) -> u8 {
    ((skills.speaking as u32
        + skills.writing as u32
        + skills.reading as u32
        + skills.listening as u32)
        / 4) as u8
}

fn get_overall_level_description(level: u8) -> &'static str {
    match level {
        0..=20 => "Beginner - Can understand and use basic phrases",
        21..=40 => "Elementary - Can handle simple communications",
        41..=60 => "Intermediate - Can handle most situations with reasonable accuracy",
        61..=80 => "Advanced - Can express fluently and spontaneously",
        81..=95 => "Proficient - Near-native level of expression",
        96..=100 => "Native - Native or bilingual proficiency",
        _ => "Unknown Level",
    }
}
