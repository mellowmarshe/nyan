use serenity::framework::standard::macros::group;

pub mod create;
pub use create::*;

pub mod profile;
pub use profile::*;

#[group]
#[commands(create, profile)]
struct Cats;
