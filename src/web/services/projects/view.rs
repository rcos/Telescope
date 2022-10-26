use crate::templates::page::Page;
use crate::templates::tags::Tags;
use crate::templates::Template;
use actix_web::web::Path;
use actix_web::HttpRequest;
use crate::error::TelescopeError;
use crate::api::rcos::projects::get_by_id::{project::ProjectProject, Project};

const TEMPLATE_PATH: &'static str = "projects/page";

#[get("/project/{project_id}")]
pub async fn project(
    req: HttpRequest,
    Path(project_id): Path<i64>,
) -> Result<Page, TelescopeError> {
    let project: Option<ProjectProject> = Project::get(project_id).await?;

    if project.is_none() {
        return Err(TelescopeError::resource_not_found(
                "Project Not Found",
                "Could not find a Project for this ID.",
                ));
    }

    let project = project.unwrap();


    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "project": &project,
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

