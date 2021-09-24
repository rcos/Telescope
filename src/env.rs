use oauth2::{ClientId, ClientSecret};
use std::sync::Arc;
use std::{collections::HashMap, env, path::PathBuf};
use std::{fs::File, io::Read, process::exit};
use structopt::StructOpt;

/// Credentials granted by GitHub for the OAuth application.
/// Generated these by creating an application at
/// <https://github.com/settings/applications/new/>.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubOauthConfig {
    /// The GitHub OAuth application client id.
    pub client_id: ClientId,
    /// The GitHub OAuth application client secret.
    pub client_secret: ClientSecret,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    /// The Discord application client id.
    pub client_id: ClientId,

    /// The Discord OAuth2 application client secret.
    pub client_secret: ClientSecret,

    /// The bot token granted by discord used to authenticate with the discord
    /// bot API.
    pub bot_token: String,

    /// The URL that Telescope is running at (to build links in discord embeds.)
    pub telescope_url: String,

    /// The discord Guild IDs for the bot to add the commands to as needed.
    /// This bot only adds commands to guilds to avoid being used outside of RCOS
    /// approved servers.
    pub guild_ids: Vec<String>,
}

/// The config of the server instance.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct TelescopeConfig {
    /// Set the log level.
    /// See <https://docs.rs/env_logger/0.8.1/env_logger/> for reference.
    log_level: Option<String>,

    /// GitHub OAuth application credentials.
    github_credentials: Option<GithubOauthConfig>,

    /// Discord application config and credentials.
    discord_config: Option<DiscordConfig>,

    /// The URL of the RCOS central API (in the OpenAPI Spec via RCOS-data).
    api_url: Option<String>,

    /// The JWT secret used to authenticate with the central API.
    jwt_secret: Option<String>,

    /// Profiles. These can be used and specified at runtime to override values
    /// defined globally. Profiles are scoped and can have sub profiles.
    profile: Option<HashMap<String, TelescopeConfig>>,
}

/// A concrete config found by searching the specified profile and parents
/// for items from the narrowest up.
///
/// The fields of this struct should match up closely to the fields of the
/// TelescopeConfig struct.
#[derive(Serialize, Debug)]
pub struct ConcreteConfig {
    /// The log level. Private because the logger is initialized in this module.
    log_level: String,
    /// The GitHub OAuth Application Credentials.
    pub github_credentials: GithubOauthConfig,
    /// The Discord Config and Credentials.
    pub discord_config: DiscordConfig,
    /// The url of the RCOS API that telescope will read and write to.
    pub api_url: String,
    /// The JWT secret used to authenticate with the central API.
    pub jwt_secret: String,
}

impl TelescopeConfig {
    /// Make the profile concrete by reverse searching profiles.
    fn make_concrete(&self, profile: Vec<String>) -> ConcreteConfig {
        // check profile exists.
        let mut scope = self;
        for part in &profile {
            if scope
                .profile
                .as_ref()
                .map(|map| map.contains_key(part))
                .unwrap_or(false)
            {
                scope = scope.profile.as_ref().unwrap().get(part).unwrap();
            } else {
                eprintln!(
                    "Profile path {:?} not found in config. missing part {}.",
                    profile, part
                );
                exit(1)
            }
        }

        let profile_slice = &profile[..];
        ConcreteConfig {
            log_level: self
                .reverse_lookup(profile_slice, |c| c.log_level.clone())
                .expect("Could not resolve log level."),
            github_credentials: self
                .reverse_lookup(profile_slice, |c| c.github_credentials.clone())
                .expect("Could not resolve GitHub OAuth credentials."),
            discord_config: self
                .reverse_lookup(profile_slice, |c| c.discord_config.clone())
                .expect("Could not resolve Discord credentials"),
            api_url: self
                .reverse_lookup(profile_slice, |c| c.api_url.clone())
                .expect("Could not resolve RCOS central API URL."),
            jwt_secret: self
                .reverse_lookup(profile_slice, |c| c.jwt_secret.clone())
                .expect("Could not resolve JWT secret."),
        }
    }

    /// Reverse lookup a property using an extractor.
    ///
    /// Assume profile is valid and exists.
    fn reverse_lookup<T: Clone>(
        &self,
        profile_slice: &[String],
        extractor: impl Fn(&Self) -> Option<T> + Copy,
    ) -> Option<T> {
        if profile_slice.len() >= 2 {
            let child_path = &profile_slice[1..];
            let child = self
                .profile
                .as_ref()
                .unwrap()
                .get(&profile_slice[0])
                .unwrap();
            // Recursively call the reverse lookup into the child profile.
            // This will resolve the deepest profile first, down to the
            // shallowest one.
            child
                .reverse_lookup(child_path, extractor)
                .or(extractor(self))
        } else if profile_slice.len() == 1 {
            extractor(
                self.profile
                    .as_ref()
                    .unwrap()
                    .get(&profile_slice[0])
                    .unwrap(),
            )
            .or(extractor(self))
        } else {
            extractor(self)
        }
    }
}

// The name, about, version, and authors are given by cargo.
/// Stores the configuration of the telescope server. An instance of this is created and stored in
/// a lazy static before the server is launched.
#[derive(Debug, Serialize, StructOpt)]
#[structopt(about = "The RCOS webapp", rename_all = "screaming-snake")]
struct CommandLine {
    /// The config file for this Telescope instance. See config_example.toml
    /// for more details.
    #[structopt(short = "c", long = "config", env, default_value = "config.toml")]
    config_file: PathBuf,
    /// What profile (if any) to use from the config file.
    ///
    /// Subprofiles can be specified using a '.' delimiter, e.g.
    /// 'dev.local'
    #[structopt(short = "p", long = "profile", env)]
    profile: Option<String>,
}

lazy_static! {
    /// Global web server configuration.
    pub static ref CONFIG: Arc<ConcreteConfig> = Arc::new(cli());
}

/// After the global configuration is initialized, log it as info.
pub fn init() {
    let cfg: &ConcreteConfig = &*CONFIG;

    // initialize logger.
    env_logger::builder().parse_filters(&cfg.log_level).init();

    info!("Starting up...");
    info!("telescope {}", env!("CARGO_PKG_VERSION"));
    trace!("Config: \n{}", serde_json::to_string_pretty(cfg).unwrap());
}

/// Get the global configuration.
pub fn global_config() -> Arc<ConcreteConfig> {
    CONFIG.clone()
}

/// Digest and handle arguments from the command line. Read arguments from environment
/// variables where necessary. Construct and return the configuration specified.
/// Initializes logging and returns config.
fn cli() -> ConcreteConfig {
    // Set env vars from a ".env" file if available.
    dotenv::dotenv().ok();

    // Get the command line args.
    let commandline: CommandLine = CommandLine::from_args();

    // Read the config file into a string.
    let mut confing_file_string = String::new();
    File::open(&commandline.config_file)
        .map_err(|e| {
            eprintln!(
                "Could not open config file at {}: {}",
                commandline.config_file.display(),
                e
            );
            e
        })
        .unwrap()
        .read_to_string(&mut confing_file_string)
        .map_err(|e| {
            eprintln!(
                "Could not read config file at {}: {}",
                commandline.config_file.display(),
                e
            );
            e
        })
        .unwrap();

    // Parse the config file into an object.
    let parsed = toml::from_str::<TelescopeConfig>(confing_file_string.as_str())
        .map_err(|e| {
            eprintln!("Error deserializing config file: {}", e);
            e
        })
        .unwrap();

    // Extract the profile from the command line args or default to empty.
    let profile_path: Vec<String> = commandline
        .profile
        .map(|s| s.split(".").map(|p| p.to_string()).collect())
        .unwrap_or(Vec::new());

    return parsed.make_concrete(profile_path);
}
