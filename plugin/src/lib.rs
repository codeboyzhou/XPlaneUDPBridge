mod logger;
mod plugin;
mod string;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use tracing::info;

const SUCCESS: c_int = 1;

#[unsafe(no_mangle)]
pub extern "C" fn XPluginStart(
    plugin_name: *mut c_char,
    plugin_signature: *mut c_char,
    plugin_description: *mut c_char,
) -> c_int {
    logger::init();
    info!("{} starting...", plugin::NAME);
    string::copy(&CString::new(plugin::NAME).unwrap(), plugin_name);
    string::copy(&CString::new(plugin::SIGNATURE).unwrap(), plugin_signature);
    string::copy(&CString::new(plugin::DESCRIPTION).unwrap(), plugin_description);
    info!("{} started successfully", plugin::NAME);
    SUCCESS
}
