# Dioxus Form From Zero

A simple login form built with [Dioxus](https://dioxuslabs.com/) — a Rust-based full-stack UI framework — demonstrating modular Rust code, asynchronous form handling, and type-safe client-server communication.

---

## Project Structure

```
src/
├── main.rs       # Entrypoint setting up routing, styling, and app launch
├── auth.rs       # Authentication logic and server-side login function
└── home.rs       # Home page component with the login form

assets/
└── style.css     # CSS styles for the app
```

---

## Overview

This example covers:

- Modular Rust project structure for maintainability  
- `#[server]` async decorated login function with automatic RPC support  
- Login form handling with uncontrolled components for performance  
- Generic form data extraction function using Rust iterators  
- Asynchronous submission preventing page reloads  
- Type-safe serialization/deserialization with Serde for client-server comms  
- Logging login attempts and results in the browser console

---

## Code Highlights

### Authentication Module (`auth.rs`)

- `LoginForm` struct with `username` and `password`  
- Constructor `new` for ergonomic instance creation  
- `#[server]` async `login` function validating credentials and returning results

### Home Component (`home.rs`)

- `Home` component rendering the login form  
- `onsubmit` handler preventing default reload and asynchronously calling login  
- Uses generic `extract_text` to parse form input values efficiently  
- Logs success or error messages in browser console

### Main Entry (`main.rs`)

- App entry point setting up routing and CSS  
- Loads global stylesheet via Dioxus asset system  
- Launches the app using `dioxus::launch`

---

## Common Questions:

- *Why use uncontrolled components?*  
  > They efficiently handle simple form input without extra re-renders or memory overhead.

- *Explain the `#[server]` macro.*  
  > It auto-generates client-server RPC code with seamless serialization and async support.

- *Why generic `extract_text`?*  
  > To accept any iterator and avoid unnecessary allocations, following idiomatic Rust patterns.

- *Benefits of modular `auth.rs` and `home.rs`?*  
  > Clear separation of concerns improves maintainability and clarity.

- *Why is serialization important?*  
  > It ensures robust, type-safe communication over the network in full-stack Rust apps.

---

## Getting Started

1. Install Rust and Cargo  
2. Clone the repository  
3. Run `cargo run` or `dx serve` if you have dioxus-cli installed  
4. Open your browser to `http://localhost:8080` (default) to see the app  

---

Made with ❤️ using Rust and Dioxus.

---

## Resources

- Official Dioxus: https://dioxuslabs.com/  
- Serde Serialization: https://serde.rs/  
- Rust async book: https://rust-lang.github.io/async-book/  
