#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  nuvio369_crm_mobile::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // no-op for wasm in the backend crate
}
