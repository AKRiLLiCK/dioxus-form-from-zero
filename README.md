# Dioxus Form From Zero

A minimal Rust fullstack web app using [Dioxus](https://dioxuslabs.com/) to demonstrate how to create and handle forms from scratch, including client-side console logging and server functions.

## Features

- Basic routing with `dioxus_router`
- Simple login form with username and password fields
- Async form submission with server function handling login logic
- Client-side logging to browser console using `web_sys::console::log_1`
- Minimal CSS for neat styling (optional to extend)
- Prevents default form submit to avoid page reload

## Getting Started

### Prerequisites

- Rust and Cargo installed: https://www.rust-lang.org/tools/install
- Optional: `dioxus-cli` for easier development (`cargo install dioxus-cli`)

### Build and Run

1. Clone the repository:

```
git clone https://github.com/YOUR_USERNAME/dioxus-form-from-zero.git
cd dioxus-form-from-zero
```

2. Run the app locally (for web target):

```
cargo install trunk  # if you want to use trunk for WASM builds
trunk serve
```

Alternatively:

```
cargo run
```

3. Open your browser at http://localhost:8080 to see the form.

### Usage

- Enter username and password.
- On submit, console logs will show the login attempt and result.
- Server function checks for username `"admin"` and password `"password"` to allow login.

## File Structure

- `src/main.rs`: Main app code, form UI, routing, server functions.
- `assets/style.css`: Optional CSS for styling the form.
- `Cargo.toml`: Rust dependencies including Dioxus and serde.

## Tips

- Open your browser developer console to see client-side logs.
- Customize the form and server logic to extend functionality.
- Use `evt.prevent_default()` in your submit handler to avoid page reload.

## License

MIT License. See [LICENSE](LICENSE) for details.

---

Made with ❤️ using Rust and Dioxus.
