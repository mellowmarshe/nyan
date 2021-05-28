use serenity::framework::standard::macros::group;

pub mod create;
pub use create::*;

pub mod profile;
pub use profile::*;

pub mod travel;
pub use travel::*;

#[group]
#[commands(create, profile, travel)]
struct Cats;
