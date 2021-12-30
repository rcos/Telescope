use crate::env::global_config;

#[derive(Serialize, Clone)]
pub struct Tags {
    pub title: String,
    #[serde(rename = "type")]
    pub og_type: String,
    pub url: String,
    pub description: String,
    pub image: String,
    pub site_name: String
}

impl Tags {
    pub fn default() -> Self {
        Tags {
            title: "Rensselaer Center for Open Source".to_string(),
            og_type: "website".to_string(),
            url: global_config().discord_config.telescope_url.clone(),
            description: "The Rensselaer Center for Open Source - or RCOS (ar-kos) for short - is a community of motivated students at Rensselaer Polytechnic Institute who develop open source projects under the guidance of experienced instructors and student mentors.".to_string(),
            image: format!("{}/{}", global_config().discord_config.telescope_url, "static/icons/rcos-branding/img/logo-square-red.png"),
            site_name: "Telescope".to_string()
        }
    }
}
