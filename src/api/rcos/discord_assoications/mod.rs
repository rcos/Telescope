//! GraphQL types queries and mutations related to entities on the RCOS discord server.

<<<<<<< HEAD
pub mod project_info;
pub mod small_group_info;
pub mod create_project_channel;
pub mod create_project_role;
pub mod create_small_group_channel;
pub mod create_small_group_role;
=======
pub mod discord_generate;
pub mod project_info;
pub mod create_channel;
pub mod create_role;
>>>>>>> f746ca16d56c965efc2a96e3ae4e91bf91df1971
/// Type representing the different kinds of channels that can be associated with a small
/// group or a project.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// A Discord voice channel.
    DiscordVoice,
    /// A Discord text channel.
    DiscordText,
    /// A Discord category channel.
    DiscordCategory,
}
