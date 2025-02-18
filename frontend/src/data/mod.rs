use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub about: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: i32,
    pub platform: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Experience {
    pub id: i32,
    pub company: String,
    pub position: String,
    pub period: String,
    pub description: String,
}

lazy_static! {
    pub static ref STATIC_PROFILE: Profile = Profile {
        name: String::from("Rasul Ramixanov"),
        description: String::from("Full-Stack Developer specializing in mobile (Flutter) and backend development."),
        about: String::from("I have been working as a Flutter mobile developer for over 4 years, gaining valuable experience in designing and building mobile applications. In addition to my expertise in Flutter, I have junior-level skills in backend development and an interest in developing iOS applications using the Swift programming language. I am passionate about learning new technologies and continuously improving my skills to deliver high-quality software solutions."),
    };

    pub static ref STATIC_PROJECTS: Vec<Project> = vec![
        Project {
            id: 1,
            name: String::from("simple-app-cache-manager"),
            description: String::from("Flutter Package for Managing Application Cache."),
            link: String::from("https://github.com/therasuldev/simple-app-cache-manager"),
        },
        Project {
            id: 2,
            name: String::from("interview-helper"),
            description: String::from("Explore programming interview questions and related books in our app. Made with Go_Router + Bloc & Flutter_Bloc + Onesignal_Flutter + Hive 💙"),
            link: String::from("https://github.com/therasuldev/Interview-helper"),
        },
        Project {
            id: 3,
            name: String::from("iNotes"),
            description: String::from("A note app with a rich-text editor built using Flutter and Rust, allowing users to style their notes with colors, bold text, and more. It’s fast, secure, and works seamlessly on both Android and iOS!"),
            link: String::from("https://github.com/therasuldev/iNotes"),
        },
    ];

    pub static ref STATIC_CONTACTS: Vec<Contact> = vec![
        Contact {
            id: 1,
            platform: String::from("GitHub"),
            link: String::from("https://github.com/therasuldev"),
        },
        Contact {
            id: 2,
            platform: String::from("LinkedIn"),
            link: String::from("https://www.linkedin.com/in/therasuldev/"),
        },
        Contact {
            id: 3,
            platform: String::from("Email"),
            link: String::from("rasul.ramixanov@gmail.com"),
        },
        Contact {
            id: 4,
            platform: String::from("X"),
            link: String::from("https://x.com/therasuldev"),
        },
        Contact {
            id: 5,
            platform: String::from("Youtube"),
            link: String::from("https://www.youtube.com/@codewithrasul"),
        },
    ];

    pub static ref STATIC_EXPERIENCES: Vec<Experience> = vec![
        Experience {
            id: 2,
            company: String::from(".ini"),
            position: String::from("Flutter developer"),
            period: String::from("2024.09 - 2025.06"),
            description: String::from("Working at .ini, I developed expertise in implementing advanced features such as maps, notifications, and complex UI designs. Additionally, I gained hands-on experience contributing to the development of over 8 cargo, courier, and shopping applications."),
        },

        Experience {
            id: 1,
            company: String::from("Nextips"),
            position: String::from("Flutter developer"),
            period: String::from("2023.10 - 2024.05"),
            description: String::from("By working at the Nextips startup, I gained experience in integrating in-app payment systems, implementing notifications, utilizing Firebase services, and building complex mobile UI designs."),
        },

    ];
}
