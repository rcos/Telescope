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

This database is served by a Hasura GraphQL APIs that
allows access to some resources when unauthenticated for public access, and
allows full access to all resources when authenticated. RCOS infrastructure
cannot connect to the database directly and must interact through this API.

## Migrations

Migrations and metadata are managed using the [Hasura CLI](https://hasura.io/docs/1.0/graphql/core/hasura-cli/index.html).

## Deployment

1. Write a `.env` file with appropriate values for each key matching the ones used in the `docker-compose.yml` file.
2. Run `docker-compose up -d` to start everything.
3. While running, use `docker logs` to inspect the log output of any of the containers. 

