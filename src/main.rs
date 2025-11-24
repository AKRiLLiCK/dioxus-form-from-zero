#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::*;
use serde::{Deserialize, Serialize};


fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
enum Route {
      #[route("/")]
      Home,
}

static CSS: Asset = asset!("/assets/style.css");

fn App() -> Element {
      rsx!(
            document::Stylesheet { href: CSS }
            Router::<Route> { }
      )
}


use dioxus::events::FormValue;

fn Home() -> Element {
      let extract_text = |values: &[(String, FormValue)], name: &str| -> String {
            values
            .iter()
            .find(|(key, _)| key == name)
            .and_then(|(_, val)| match val {
                  FormValue::Text(s) => Some(s.clone()),
                  _ => None,
            })
            .unwrap_or_default()
      };

      rsx!(
      h1 { "My first form" }
      form {
            onsubmit: move |evt| {
                  evt.prevent_default(); // Added this line to prevent page reload
                  async move {
                        let values_vec: Vec<(String, FormValue)> = evt.values().into_iter().collect();
                        
                        let username = extract_text(&values_vec, "username");
                        let password = extract_text(&values_vec, "password");
                        
                        web_sys::console::log_1(&format!("Attempt login with username={}, password={}", username, password).into());
                        
                        let data = LoginForm { username, password };
                        
                        match login(data).await {
                              Ok(()) => web_sys::console::log_1(&"Login successful".into()),
                              Err(e) => web_sys::console::log_1(&format!("Login failed: {}", e).into()),
                        }
                  }
            },

            label { "Username:" }
            input { r#type: "text", name: "username", required: true }
            br {}
            label { "Password:" }
            input { r#type: "password", name: "password", required: true }
            br {}
            button { r#type: "submit", "Login" }
      }
)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoginForm {
      username: String,
      password: String,
}

#[server]
async fn login(form: LoginForm) -> Result<(), ServerFnError> {
      if form.username == "admin" && form.password == "password" {
            println!("Login successful");
            Ok(())
      } else {
            println!("Login failed");
            Err(ServerFnError::new("Invalid credentials"))
      }
}