mod logger;
mod plugin;
mod safe;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use tracing::info;

#[unsafe(no_mangle)]
pub extern "C" fn XPluginStart(
    plugin_name: *mut c_char,
    plugin_signature: *mut c_char,
    plugin_description: *mut c_char,
) -> c_int {
    logger::init();
    info!("{} starting...", plugin::NAME);
    safe::write_c_char(plugin_name, &CString::new(plugin::NAME).unwrap());
    safe::write_c_char(plugin_signature, &CString::new(plugin::SIGNATURE).unwrap());
    safe::write_c_char(plugin_description, &CString::new(plugin::DESCRIPTION).unwrap());
    info!("{} started successfully", plugin::NAME);
    plugin::STARTED
}
