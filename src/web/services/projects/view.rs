use crate::templates::page::Page;
use crate::templates::tags::Tags;
use crate::templates::Template;
use actix_web::web::Path;
use actix_web::HttpRequest;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::Identity;
use crate::api::rcos::projects::get_by_id::{project::ProjectProject, Project};
use crate::api::rcos::projects::authorization_for::{AuthorizationFor, UserProjectAuthorization};


const TEMPLATE_PATH: &'static str = "projects/page";

#[get("/project/{project_id}")]
pub async fn project(
    req: HttpRequest,
    Path(project_id): Path<i64>,
    identity: Identity,
) -> Result<Page, TelescopeError> {
    //get the viewer's id
    let viewer: Option<_> = identity.get_user_id().await?;

    //get their authorization level
    let authorization = AuthorizationFor::get(viewer).await?;

    let project: Option<ProjectProject> = Project::get(project_id).await?;

    if project.is_none() {
        return Err(TelescopeError::resource_not_found(
                "Project Not Found",
                "Could not find a Project for this ID.",
                ));
    }

    let project = project.unwrap();

    let can_edit = authorization.can_edit();

    if !authorization.can_view() {
        return Err(TelescopeError::BadRequest {
            header: "Project Not Visible".into(),
            message: "You do not have permission to view this project".into(),
            show_status_code: false,
        })
    }

    let mut template = Template::new(TEMPLATE_PATH);

    template.fields = json!({
        "project": &project,
        "auth": authorization,
    });

    let mut tags = Tags::default();
    // Set title and URL trivially.
    tags.title = project.title();
    dbg!(&tags.title);
    tags.url = req.uri().to_string();

    // tags.description = project.description.clone();
    // dbg!(&tags.description);

    // Build page around meeting template.
    let mut page = template.in_page(&req, project.title()).await?;
    // Replace default page tags with meeting specific ones.
    page.ogp_tags = tags;
    // Return page.
    return Ok(page);
}

