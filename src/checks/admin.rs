use log::error;
use serenity::{
    client::Context,
    framework::standard::{
        macros::check,
        Args,
        CommandOptions,
        Reason,
    },
    model::prelude::*,
};

#[check]
#[name("Admin")]
pub async fn admin_check(
    ctx: &Context,
    msg: &Message,
    _args: &mut Args,
    _opts: &CommandOptions,
) -> Result<(), Reason> {
    if let Some(guild) = msg.guild(&ctx.cache).await {
        let channel = match guild.channels.get(&msg.channel_id) {
            Some(channel) => channel,
            None => return Err(Reason::Unknown),
        };
        let member = match msg.member(ctx).await {
            Ok(member) => member,
            Err(e) => {
                return Err(Reason::User(format!("Failed to fetch member info: {}", e)));
            }
        };
        let perms = match guild.user_permissions_in(channel, &member) {
            Ok(perms) => perms,
            Err(e) => {
                error!(
                    "Error getting permissions for user {} in channel {}: {}",
                    member.user.id, channel.id, e
                );
                return Err(Reason::Unknown);
            }
        };

        if perms.administrator() {
            Ok(())
        } else {
            Err(Reason::User("Not Admin".to_string()))
        }
    } else {
        // User is probably in a DM.
        Ok(())
    }
}
