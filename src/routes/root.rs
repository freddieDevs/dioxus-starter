use dioxus::prelude::*;

use crate::components::logo::Logo;

#[component] 
pub fn Root() -> Element {
  //navigate
  //data
  //clusters
  //loading

  rsx! {
    div {
      class: "bg-amber-100/90",
      div {
        class: "flex ml-8 pb-4 border-b-2 justify-between md:justify-center items-center text-cyan-800",
        Logo {}
        div {
          class: "md:border-l border-slate-300 px-4",
          h2 {
            class: "hidden md:block text-3xl font-bold tracking-tight lg:text-4xl",
            "Passionate Road Traffic Safety and Food Chama Empowerment",
          }
          h2 {
            class: "md:hidden text-4xl font-bold tracking-wide",
            "PASFCE"
          }
        }
        div {
          class: "md:hidden",
          "mobile navbar component to pass cluster"
        }
      }
      div {
        class: "flex h-[100vh] w-screen mb-5 py-6",
        div {
          class: "hidden md:flex flex-col gap-y-2 h-full w-[250px] px-2 border-r-2",
          "Sidebar that we pass data to"
        }
        div {
          class: "flex-1 overflow-y-auto px-6",
          "render outlet to show everything else"
        }
      }
    }
  }
}