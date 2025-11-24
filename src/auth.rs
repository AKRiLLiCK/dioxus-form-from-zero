use super::*;
#[derive(Deserialize, Serialize, Clone)]
pub struct LoginForm {
      username: String,
      password: String,
}

impl LoginForm {
      pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
            let username: String = username.into();
            let password: String = password.into();

            Self { username, password }
      }
}

#[server]
pub async fn login(form: LoginForm) -> Result<(), ServerFnError> {
      if form.username == "admin" && form.password == "password" {
            println!("Login successful");
            Ok(())
      } else {
            println!("Login failed");
            Err(ServerFnError::new("Invalid credentials"))
      }
}