use ::dioxus::prelude::*;
use ::dioxus_router::*;
use ::serde::{Deserialize, Serialize};
use home::Home;

mod home;
mod auth;

static CSS: Asset = asset!("/assets/style.css");

#[derive(Routable, Clone)]
enum Route {
      #[route("/")]
      Home
}

#[component]
fn App() -> Element {
      rsx!(
            document::Stylesheet { href: CSS }
            Router::<Route> { }
      )
}

fn main() {
    ::dioxus::launch(App);
}