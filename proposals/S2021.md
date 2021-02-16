# RCOS Infrastructure Spring 2021

## Mission Statement
The Spring 2021 RCOS infrastructure team is comprised of three coordinators 
and three repositories and aims to unify efforts to modernize and stabilize
RCOS documentation and tooling. 

## Team
- Nia Calia-Bogan (Telescope PM)
    - [GitHub](https://github.com/Alfriadox)
    - RPI email: caliaa2@rpi.edu
    - Other email: acaliabogan@acaliabogan.dev
- Steven vanZyl
    - [GitHub](https://github.com/rushsteve1)
    - RPI email: vansys@rpi.edu
- Frank Matranga
    - [GitHub](https://github.com/Apexal)
    - RPI email: matraf@rpi.edu

## Projects
### Telescope 
Links:
- [GitHub repo](https://github.com/rcos/Telescope)
- [eventual website](https://rcos.io)

Stack:
Telescope is written in [rust](https://www.rust-lang.org/) using the 
[actix](https://actix.rs/) framework and the [handlebars](https://handlebarsjs.com/) 
templating engine.

Description:
Telescope aims to improve on and replace the current RCOS website
for tracing student engagement. Our current website,
[Observatory](https://github.com/rcos/observatory-server) is outdated and not 
maintained. Telescope aims to replace observatory within the Spring 2021 semester
and bring a variety of new features such as QR-code attendance and better 
GitHub integration. Telescope is in its third semester of development and is
being reworked this semester to use the central RCOS database and API. Despite this
we aim to deploy the first version near the end of this semester if possible.

### RCOS Database
Links:
- [GitHub repo](https://github.com/rcos/rcos-data)
- [Public API](https://api.rcos.io)
- [Public API Playground](https://swagger.rcos.io)

Stack: 
The RCOS central database is a [Postgres](https://www.postgresql.org/) database
that exposes an API in the OpenAPI 2.0 spec using 
[PostgREST](https://postgrest.org/en/v7.0.0/#).

Description:
The rcos-data project aims to centralize RCOS data and provide a convenient 
and stable API to access it. It has been deployed as of February 2021 and is 
being maintained and improved by the infrastructure team according to the needs 
of Telescope and the rest of RCOS. This database aims to be a long-term stable 
solution to be passed down to future coordinators for the foreseeable future. 

### RCOS Handbook
Links:
- [GitHub repo](https://github.com/rcos/rcos-handbook)
- [Website](https://handbook.rcos.io)

Stack: 
The RCOS handbook is built using [Docsify.js](https://docsify.js.org/#/).

Description:
The RCOS handbook is a central location for documentation about participating 
in RCOS and the rules and requirements for students, mentors, and coordinators.
The handbook has been around for years, but has been largely unused and unmaintained 
prior to this semester. The Spring 2021 infrastructure team aims to update the 
handbook and bring it back into use. 

## Milestones:
Software developers are infamously bad at predicting the timeline of a project
but we make an attempt to do so here anyways.
- Week 1 (Feb 16 - 22):
    - Authentication via GitHub OAuth
    - API queries and models for users, user accounts, projects, and semesters
- Week 2 (Feb 23 - Mar 1):
    - User profile pages
    - Project pages
- Week 3 (Mar 2 - 8):
    - User setting pages
    - Project settings pages
- Week 4 (Mar 9 - 15):
    - Small group models and API queries
    - Meetings models and API queries
- Week 5 (Mar 16 - 22):
    - Mentor pages
    - Coordinator pages
    - Administrator pages
- Week 6 (Mar 23 - 29):
    - Status update models and queries
    - Status update pages
- Week 7 (Mar 30 - Apr 5):
    - Attendance models and queries
    - Attendance pages
- Week 8 (Apr 6 - 12):
    - Project proposal & presentation models and queries
    - Workshop proposal models and queries
- Week 9 (Apr 13 - 19):
    - Project proposal & presentation pages.
    - Calendar page
    - Workshop page
- Week 10 (Apr 20 - 26):
    - Final Tweaks
    - Deploy to [rcos.io](https://rcos.io).
- Week 11 (April 27 - May 3):
    - Iron out any issues with deployment.
    - Request user feedback from current RCOS students.
    - Final presentation.
