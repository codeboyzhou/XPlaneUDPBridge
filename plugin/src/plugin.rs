use std::os::raw::c_int;

pub(crate) const STARTED: c_int = 1;

pub(crate) const XPLANE_C_CHAR_BUFFER_SIZE: usize = 256;

pub(crate) const NAME: &str = "xplane-udp-bridge";

pub(crate) const SIGNATURE: &str = "https://github.com/codeboyzhou/xplane-udp-bridge";

pub(crate) const DESCRIPTION: &str =
    "This plugin allows you to communicate with X-Plane 12 via UDP.";
