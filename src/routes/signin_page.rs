use dioxus::prelude::*;
use dioxus_logger::tracing;
use crate::ui::form::{use_form_context, FormProvider};
use crate::ui::form_field::FormField;

#[component]
pub fn SigninPage() -> Element {
  //let form = use_form_context();

  rsx! {
    div {
      class: "fixed inset-0 z-50 bg-background/50 backdrop-blur-sm",
      div {
        class: "flex items-center justify-center h-screen",
        div {
          class: "fixed z-50 grid w-full space-y-4 max-w-lg mx-auto border rounded-lg p-4 shadow-lg bg-amber-100 text-cyan-900",
          h1 {
            class: "text-center text-2xl font-bold",
            "Sign In"
          }
          div {
            class: "space-y-4 py-2 pb-4",
            FormProvider {
              form {
                  label {
                    class: "text-sm",
                    "Username: "}
                  FormField {
                    field_name: "username",
                    placeholder: "Enter your Username",
                    input_type: "text",
                  }
                
                  label { 
                    class: "text-sm",
                    "Password: "}
                  FormField {
                    field_name: "password",
                    placeholder: "Enter your Password",
                    input_type: "password",
                  }
                
              }
              div {
                class: "pt-6 space-x-2 flex items-center justify-end w-full",
                SubmitButton {}
              }
            }
          }
        }
      }
    }
  }
}

#[component]
fn SubmitButton() -> Element {
  let form = use_form_context();
  
  rsx! {
    button {
      class: "bg-cyan-800 text-accent hover:bg-cyan-900 rounded px-4",
      r#type: "submit",
      onclick: move |_| {
        if let Ok(handle_submit) = form.handle_submit.try_borrow() {
          handle_submit();
        } else {
          tracing::info!("failed to access the handle_submit closure");
        }
      },
      
      "Submit"
    }
  }
}