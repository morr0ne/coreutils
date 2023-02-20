#[cfg(feature = "arch")]
mod arch;
#[cfg(feature = "b2sum")]
mod b2sum;
#[cfg(feature = "basename")]
mod basename;
#[cfg(feature = "true")]
mod r#true;
#[cfg(feature = "tty")]
mod tty;
#[cfg(feature = "uname")]
mod uname;
#[cfg(feature = "who")]
mod who;
#[cfg(feature = "whoami")]
mod whoami;
#[cfg(feature = "yes")]
mod yes;

#[cfg(feature = "arch")]
pub use arch::arch;
#[cfg(feature = "b2sum")]
pub use b2sum::b2sum;
#[cfg(feature = "true")]
pub use r#true::r#true;
#[cfg(feature = "tty")]
pub use tty::tty;
#[cfg(feature = "uname")]
pub use uname::uname;
#[cfg(feature = "whoami")]
pub use whoami::whoami;
#[cfg(feature = "yes")]
pub use yes::yes;