pub mod app;
pub mod catalog;
pub mod compute;
pub mod format;
pub mod views;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  leptos::mount::hydrate_body(app::App);
}
