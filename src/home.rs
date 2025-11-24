use super::*;

#[component]
pub fn Home() -> Element {
      rsx!(
            h1 { "My first form" }
            form {
                  onsubmit: move |evt| {
                        evt.prevent_default(); // THIS LINE PREVENTS PAGE RELOAD
                        async move {
                              
                              let username = extract_text(evt.values(), "username");
                              let password = extract_text(evt.values(), "password");
                              
                              ::web_sys::console::log_1(&format!("Attempt login with username={}, password={}", username, password).into());
                              
                              let data = auth::LoginForm::new(username, password);
                        
                              match auth::login(data).await {
                                    Ok(()) => ::web_sys::console::log_1(&"Login successful".into()),
                                    Err(e) => ::web_sys::console::log_1(&format!("Login failed: {}", e).into()),
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

fn extract_text<I>(values: I, name: &str) -> String
where
I: IntoIterator<Item = (String, FormValue)>,
{
      values
      .into_iter()
      .find(|(key, _)| key == name)
      .and_then(|(_, val)| match val {
            FormValue::Text(s) => Some(s),
            _ => None,
      })
      .unwrap_or_default()
}