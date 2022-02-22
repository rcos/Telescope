//! GraphQL types queries and mutations related to entities on the RCOS discord server.

pub mod discord_generate;
pub mod project_info;
pub mod create_channel;
pub mod create_role;
/// Type representing the different kinds of channels that can be associated with a small
/// group or a project.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// A Discord voice channel.
    DiscordVoice,
    /// A Discord text channel.
    DiscordText,
}
