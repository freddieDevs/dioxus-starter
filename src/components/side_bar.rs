// use dioxus::prelude::*;
// use dioxus_router::prelude::{Link, use_route};
// use std::rc::Rc;


// #[derive(PartialEq, Clone)]
// pub struct Cluster {
//   pub id: String,
//   pub name: String,
// }

// #[derive(Props, Clone, PartialEq)]
// pub struct SidebarProps {
//   data: Option<Rc<Vec<Cluster>>>,
// }

// #[component] 
// pub fn Sidebar(props: SidebarProps) -> Element {
//   // let route = use_route();
//   // let cluster_id = route.params().get("cluster_id").cloned().unwrap_or_default();
//   // let pathname = route.path();

//   // let cluster_array = props.data.clone().unwrap_or_default();

//   // let routes = vec![
//   //   MatchRoute { href: format!("/{cluster_id}"), label: "Dashboard", active: pathname == format!("/{cluster_id}")},
//   //   MatchRoute { href: format!("/{cluster_id}/members"), label: "Members", active: pathname == format!("/{cluster_id}/members")},
//   //   MatchRoute { href: format!("/{cluster_id}/staffs"), label: "Staff", active: pathname == format!("/{cluster_id}/staffs")},
//   //   MatchRoute { href: format!("/{cluster_id}/savings"), label: "Saving", active: pathname == format!("/{cluster_id}/savings")},
//   //   MatchRoute { href: format!("/{cluster_id}/reports"), label: "Reports", active: pathname == format!("/{cluster_id}/reports")},
//   //];

//   rsx! {
//     div {
//       // class: "flex items-center flex-col gap-y-1",
//       // "cluster switcher",
//       // {routes.iter().map(|route| rsx! {
//       //   Link {
//       //     to: &route.href,
//       //     class: format_args!(
//       //       "hover:bg-cyan"
//       //     ),
//       //     "{route.label}"
//       //   }
//       // })}
//       "Sidebar"
//     }
//   }
// }

// #[derive(PartialEq, Clone)]
// struct MatchRoute {
//   href: String,
//   label: &'static str,
//   active: bool,
// }