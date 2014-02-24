#[crate_id = "ws#0.1-pre"];
//#[crate_type = "dylib"]; // FIXME this fails with regular rust-crypto, since just builds rlibs by default. TODO PR to fix that
#[crate_type = "rlib"];

extern mod extra;
extern mod http;
extern mod rust_crypto = "rust-crypto";

pub mod server;
pub mod message;
