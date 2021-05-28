use serenity::framework::standard::macros::group;

pub mod ping;
pub use ping::*;

pub mod help;
pub use help::*;

#[group]
#[commands(ping)]
struct Misc;
