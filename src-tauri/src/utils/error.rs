use tauri::{AppHandle, Emitter};

// should come with type and error message
pub fn emit_error(app_handle: &AppHandle, payload: &str) {
    app_handle
        .emit("error", payload)
        .expect("app handle should emit error");
}
