// There are couple things the frontend needs to know when booting
// We need to return:
//    Currently open windows
//    Currently open processes
//    Favorited Phrases
//    Current whitelist settings
//    Whether or not the app is on or not

// Return a vector of window names
#[tauri::command]
pub fn get_open_windows() -> Vec<String> {
    vec![]
}

// Return a vector of process names
#[tauri::command]
pub fn get_open_processes() -> Vec<String> {
    vec![]
}

// Return a vector of tuples of (Your message, translated message)
#[tauri::command]
pub fn get_fav_phrases() -> Vec<(String, String)> {
    vec![]
}

// Return 2 vectors, one for window names, one of processes
#[tauri::command]
pub fn get_whitelist() -> (Vec<String>, Vec<String>) {
    (vec![], vec![])
}

// Get the current on/off of the app
#[tauri::command]
pub fn get_active_status() -> bool {
    true
}

// We'll also want to be able to change the whitelist and on/off

// Whitelist a window name or remove from whitelist
// The `state` param is true to include in the whitelist, set to false to remove
// Returns whether or not action was successful or not 
#[tauri::command]
pub fn set_whitelist_window(window_name: String, state: bool) -> Result<String, String> {
    Ok("Success!".into())
}

// Whitelist a process name or remove from whitelist
// The `state` param is true to include in the whitelist, set to false to remove
// Returns whether or not action was successful or not 
#[tauri::command]
pub fn set_whitelist_process(process_name: String, state: bool) -> Result<String, String> {
    Ok("Success!".into())
}

// Turn the app on or off
// The function toggles the app on/off 
// Returns the current state of the program, true: on, false: off
#[tauri::command]
pub fn toggle_app_state() -> bool {
    true
}