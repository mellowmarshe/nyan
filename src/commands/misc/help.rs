use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, prelude::UserId},
    prelude::*,
};

use std::collections::HashSet;

#[help]
#[individual_command_tip("To get help with an individual command, pass its name as an argument to this command.\n[] = required, <> = optional")]
#[lacking_permissions("Hide")]
#[lacking_role("Hide")]
#[wrong_channel("Hide")]
#[lacking_conditions("Nothing")]
#[suggestion_text("`Did you mean {}..?`")]
#[max_levenshtein_distance(3)]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    ops: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::plain(ctx, msg, args, ops, groups, owners).await;

    Ok(())
}
