/* main.rs */

#[crate_id = "main#0.1"];
#[comment = "ironkernel"];
#[license = "MIT"];
#[crate_type = "lib"];
// Forked from pczarn/rustboot
#[no_std];
#[feature(asm, globs, macro_rules)];

extern mod core;

#[cfg(target_arch = "arm")]
pub use support::{memcpy, memmove};

use platform::{cpu, io};
pub mod kernel;

#[cfg(target_arch = "arm")]
#[path = "rust-core/support.rs"]
mod support;

#[cfg(target_arch = "arm")]
#[path = "arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}
