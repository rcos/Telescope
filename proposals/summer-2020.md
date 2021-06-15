## Telescope Summer 2020

#### Links
- [GitHub Repo](https://github.com/rcos/Telescope)
- [Mattermost channel](https://chat.rcos.io/rcos/channels/telescope)

#### Team
- Antonia "Nia" Calia-Bogan (PM, Backend, Full-Stack)
    - Mattermost: @alfriadox
    - RPI email: caliaa2@rpi.edu
    - Other email: acaliabogan@acaliabogan.dev
- Patrick Berne (Frontend)
    - Mattermost: @bernep
    - RPI email: bernep@rpi.edu

#### Description
Telescope is a website that manages projects and developers for RCOS.
We aim to replace the current system, 
[Observatory](https://github.com/rcos/observatory-server).
Key features include attendance tracking, user and project management,
and a comprehensive set of tools for mentors and coordinators. Telescope
is written in [rust](https://www.rust-lang.org/) and uses 
[actix](https://actix.rs/) for the backend and 
[handlebars](https://handlebarsjs.com/) for the frontend.

#### Milestones:
Since ARCH is a shorter semester than others we only actually have 10 weeks or
so to develop Telescope. We optimistically hope to finish telescope by the end
of the summer semester, for a Fall release. The predicted development schedule
is as follows:
- Week 0 (June 7 - 13):
    - Create GitHub Repo
    - Project proposal
    - Finalize team
    - Initialize project
    - TLS/SSL support
    - Static page support
- Week 1 (June 14 - 20):
    - Database support
    - Cookies and session tokens
- Week 2 (June 21 - 27):
    - Authentication 
    - Login Page
- Week 3 (June 28 - July 4):
    - Summer Break - No Classes
- Week 4 (July 5 - 11):
    - Backend infrastructure for users
- Week 5 (July 12 - 18):
    - Frontend Pages for users
    - Backend infrastructure for projects
- Week 6 (July 19 - 25):
    - Frontend pages for projects
    - Backend infrastructure for attendance tracking
- Week 7 (July 26 - August 1):
    - Frontend pages for attendance tracking.
    - Achievements, Sponsors, and Home pages.
- Week 8 (August 2 - 8):
    - Debugging & Polishing week
    - Finish any unfinished features
- Week 9 (August 9 - 15):
    - Deploy to [rcos.io](https://rcos.io).
    - Production database migration.
- Week 10 (August 16 - 21):
    - Final presentation.