use serenity::framework::standard::macros::group;

pub mod treats;
pub use treats::*;

#[group]
#[commands(treats)]
struct Treats;
