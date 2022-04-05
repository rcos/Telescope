//! GraphQL types queries and mutations related to entities on the RCOS discord server.
/// Type representing the different kinds of channels that can be associated with a small
/// group or a project.
pub mod project;
pub mod small_group;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// A Discord voice channel.
    DiscordVoice,
    /// A Discord text channel.
    DiscordText,
}
