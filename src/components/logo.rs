use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarImage};

#[component] 
pub fn Logo() -> Element {
  const LOGO: Asset = asset!("/public/pasfce.png");
  rsx! {
    div {
      class: "flex w-40 h-40 mr-4 rounded-full items-center justify-center",
      Avatar {
        class: Some("flex w-28 h-28"),
        AvatarImage {
          src: LOGO,
          alt: "Company Logo",
          class: Some("flex w-full h-full object-fit")
        },
        // AvatarFallback {
        //   fallback_text: "PAS",
        //   class: Some("text-white")
        // }
      }
    }
  }
}