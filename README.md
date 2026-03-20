# 🦀 SeedRust

SeedRust is a frontend web application built entirely in Rust and compiled to WebAssembly (Wasm). It serves as a product showcase/e-commerce storefront, demonstrating how to build fast, type-safe Single Page Applications (SPAs) using the Yew framework.

This application highlights client-side rendering (CSR), routing, and asynchronous data fetching in a Rust-based frontend environment.

---

## 🚀 About The Project

The goal of this project is to build a responsive, client-rendered web application using Rust instead of traditional JavaScript/TypeScript. It fetches product data asynchronously from static JSON files, processes it using strongly-typed Rust structs, and renders a dynamic user interface. 

---

## 🛠️ Tech Stack

This project utilizes a modern WebAssembly-first tech stack:

* **Framework:** [Yew](https://yew.rs/) (v0.20 with CSR feature)
* **Language:** [Rust](https://www.rust-lang.org/)
* **Routing:** `yew-router` for client-side navigation
* **Network/HTTP:** `gloo-net` for fetching asynchronous data
* **Serialization:** `serde` & `serde_json` for robust JSON parsing
* **WebAssembly:** `wasm-bindgen` & `wasm-bindgen-futures`
* **Deployment:** [Vercel](https://vercel.com/) (configured via `vercel.json`)

---

## 🔥 Features

This application implements core single-page application features using a Rust ecosystem.

### Core Application
* **Client-Side Rendering (CSR):** The entire UI is rendered dynamically in the browser using WebAssembly.
* **Component-Based UI:** Built with modular and reusable Yew components.
* **Type-Safe Data:** Product data is safely parsed into Rust structs, preventing runtime type errors.

### Data & Routing
* **Asynchronous Fetching:** Uses `gloo-net` to fetch product lists and individual product details (`/products/products.json`).
* **Client-Side Navigation:** Seamless page transitions using `yew-router` without full page reloads.
* **Dynamic Routes:** Supports parameterized routing to display individual product pages based on IDs.

### Technical Excellence
* **High Performance:** Runs near-native speeds in the browser thanks to WebAssembly.
* **Build Tooling:** Utilizes `Cargo` and `Makefile.toml` (likely via `cargo-make` or `trunk`) for streamlined building and bundling.
* **Vercel Integration:** Ready for edge deployment with an included `vercel.json` configuration.

---

## 🏁 Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (latest stable version)
* WebAssembly target for Rust:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
* [Trunk](https://trunkrs.dev/) (the Wasm web application bundler for Rust) or `cargo-make`:
  ```sh
  cargo install trunk
  # or
  cargo install cargo-make
  ```

### Installation

1.  **Clone the repo**
    ```sh
    git clone https://github.com/sumantwarhekar/SeedRust.git
    cd SeedRust
    ```

2.  **Build and run the development server**
    Depending on your preferred bundler (Trunk is highly recommended for Yew):
    ```sh
    trunk serve
    ```
    *Alternatively, if using the included Makefile.toml:*
    ```sh
    cargo make serve
    ```

3. Open your browser and navigate to the local server address (usually [http://localhost:8080](http://localhost:8080)) to see the application running.
