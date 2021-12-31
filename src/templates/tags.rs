use crate::env::global_config;
use actix_web::HttpRequest;

/// The Open Graph Protocol tags.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tags {
    /// The page title.
    pub title: String,

    /// The OpenGraph page type. This will almost always be "website".
    #[serde(rename = "type")]
    pub og_type: String,

    /// The URL of the page.
    pub url: String,

    /// The page description.
    pub description: String,

    /// An image relevant to the page.
    pub image: String,

    /// The site name. This will almost always be "Telescope".
    pub site_name: String,
}

impl Tags {
    /// Fill the url using the one in the HTTP request.
    pub fn for_request(request: &HttpRequest) -> Self {
        Tags {
            url: request.uri().to_string(),
            // Fill remaining fields from default.
            ..Self::default()
        }
    }
}

impl Default for Tags {
    fn default() -> Self {
        Tags {
            title: "Rensselaer Center for Open Source".to_string(),
            og_type: "website".to_string(),
            url: global_config().telescope_url.clone(),
            description: "The Rensselaer Center for Open Source - or RCOS (ar-kos) for short - is \
                a community of motivated students at Rensselaer Polytechnic Institute who develop \
                open source projects under the guidance of experienced instructors and student \
                mentors."
                .to_string(),
            image: format!(
                "{}/{}",
                global_config().telescope_url,
                "static/icons/rcos-branding/img/logo-square-red.png"
            ),
            site_name: "Telescope".to_string(),
        }
    }
}
