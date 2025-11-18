mod string;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

const SUCCESS: c_int = 1;

const PLUGIN_NAME: &str = "xplane-udp-bridge";
const PLUGIN_SIGNATURE: &str = "https://github.com/codeboyzhou/xplane-udp-bridge";
const PLUGIN_DESCRIPTION: &str = "This plugin provides a UDP bridge for X-Plane 12.";

#[unsafe(no_mangle)]
pub extern "C" fn XPluginStart(
    plugin_name: *mut c_char,
    plugin_signature: *mut c_char,
    plugin_description: *mut c_char,
) -> c_int {
    string::copy(&CString::new(PLUGIN_NAME).unwrap(), plugin_name);
    string::copy(&CString::new(PLUGIN_SIGNATURE).unwrap(), plugin_signature);
    string::copy(&CString::new(PLUGIN_DESCRIPTION).unwrap(), plugin_description);
    SUCCESS
}
