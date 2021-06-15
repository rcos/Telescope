# RCOS Infrastructure Summer 2021

## Team
- Nia Calia-Bogan (Telescope PM)
    - [GitHub](https://github.com/Alfriadox)
    - RPI email: caliaa2@rpi.edu
    - Other email: acaliabogan@acaliabogan.dev

## Projects
### Telescope
#### Links: 
- [repository](https://github.com/rcos/Telescope)
- [website](https://telescope.rcos.io)

#### Stack:
Telescope is written in [rust](https://www.rust-lang.org/) using the
[actix](https://actix.rs/) framework and the [handlebars](https://handlebarsjs.com/)
templating engine.


### RCOS Database
#### Links:
- [repository](https://github.com/rcos/rcos-data)

#### Stack: 
The RCOS Database is maintained via SQL ([postgres](https://www.postgresql.org/)) migrations invoked via the 
[Hasura](https://hasura.io/) console. It exposes a [GraphQL](https://graphql.org/) API via [Hasura](https://hasura.io/)
and a legacy REST API (which may be deprecated in the future) via [Swagger](https://swagger.io/).  


## Description
This Summer, the RCOS Infrastructure team is just me. I intend to focus most of 
my time and energy on developing Telescope to an MVP and maintaining and updating 
the RCOS Database as needed. With that in mind, below is an 
optimistic timeline for this semester.

## Timeline
Week 1 (June 14 - 20):
    - Make / Rework authorization middleware.
    - Meeting Creation/Edit form.
    - Meeting deletion functionality.
Week 2 (June 21 - 27):
    - User Settings Page
    - User enrollment perhaps?
Week 3 (June 28 - July 4):
    - I will be on vacation this week and am not expecting to make much progress. 
Week 4 (July 5 - 11):
    - OAuth2 Linking / Unlinking in UI
    - Discord Server gateway
Week 5 (July 12 - 18):
    - Projects page
    - Project pages
Week 6 (July 19 - 25):
    - Project proposal form
    - Project proposal acceptance page
Week 7 (July 26 - August 1):
    - Meeting attendance page
Week 8 (August 2 - 8):
    - Mentor/Coordinator attendance tools 
Week 9 (August 9 - 15):
    - Pay requests form
    - Pay requests page
Week 10 (August 16 - 22):
    - 1.0.0 release
