# RCOS Database
> RCOS database schema and related tools.

## Overview

This repo holds the SQL code for setting up the RCOS database, the database
definition, views and triggers, and tools for importing RCOS data from external
platforms like Submitty and Venue.

## Database
The RCOS database is a Postgres DB running on our own infrastructure. Access is
restricted to coordinators and faculty advisors, but the schema and tools used
are open-sourced here.

## API

This database is served by a Postgrest API [here](https://swagger.rcos.io/#/) that
allows access to some resources when unauthenticated for public access, and
allows full access to all resources when authenticated. RCOS infrastructure
cannot connect to the database directly and must interact through this API.

## Deployment

1. Replace the `<PASSWORD>` and `<ADMINPASS>` placeholders in
   `docker-compose.yml` and the [roles migration
   file](./db/migrations/20210117194733_create_roles.sql) with secure passwords.
2. Run `docker-compose up -d` to start everything.
3. Use [DbMate](https://github.com/amacneil/dbmate) to apply the migations
   `dbmate up`.
