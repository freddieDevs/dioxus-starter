#![allow(non_snake_case)]

pub mod routes {
    pub mod signin_page;
    pub mod root;
}
pub mod ui {
    pub mod form_field;
    pub mod form;
    pub mod button;
    pub mod avatar;
}

mod components {
    pub mod logo;
}

use dioxus::prelude::*;
use dioxus_logger::tracing;

use routes::{signin_page::SigninPage, root::Root};
pub use ui::{form, form_field, button, avatar::{Avatar, AvatarImage, AvatarFallback}};


#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Root {},
    #[route("/signin")]
    SigninPage {},
    // #[route("/:clusterid")]
    // Root {}, will render the index page
    // #[route("/:clusterid/members")]
    // MembersPage {},
    // #[route("/:clusterid/members/:member_id")]
    // MemberIdPage {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    let tailwind_css = include_str!("../public/output.css");
    // TODO: add the modal and toast provider here
    rsx! {
        style { "{tailwind_css}" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::SigninPage { },
            "Go to Signin page"
        }
        div {
            h1 {
                class: "text-stone-900 bg-blue-600", "High-Five counter: {count}" }
            button {class: "rounded bg-green-400 text-white", onclick: move |_| count += 1, "Up high!" }
            button { 
                class: "rounded bg-yellow-700 text-black", 
                onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
