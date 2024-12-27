use dioxus::prelude::*;

//utility fn for conditional styling
fn cn(classes: &[&str]) -> String {
  classes.iter()
    .filter(|c| !c.is_empty())
    .cloned()
    .collect::<Vec<_>>()
    .join(" ")
}

//avatar component
#[component] 
pub fn Avatar(class: Option<&'static str>, children: Element) -> Element {
  rsx! {
    div {
      class: "{cn(&[\"relative flex h-10 shrink-0 overflow-hidden rounded-full\", class.unwrap_or(\"\")])}",
      {children}
    }
  }
}

//avatar_image component
#[component] 
pub fn AvatarImage(src: Asset, alt: &'static str, class: Option<&'static str>) -> Element {
  rsx! {
    img {
      src: "{src}",
      alt: "{src}",
      class: "{cn(&[\"aspect-square h-full w-full\", class.unwrap_or(\"\")])}"
    }
  }
}

//struct for avatarfallback
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
  pub fallback_text: &'static str,
  #[props(optional)]
  pub class: Option<&'static str>
}

//avatar_fallback component
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {

  rsx! {
    div {
      class: "{cn(&[\"flex h-full w-full items-center justify-center rounded-full bg-muted\", props.class.unwrap_or(\"\")])}",
      "{props.fallback_text}"
    }
  }
}