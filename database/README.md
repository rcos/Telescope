# RCOS Database
> RCOS database schema and related tools.

## Overview

This repo holds the SQL for setting up the RCOS database, the database definition, views and triggers, and tools for importing RCOS data from external platforms like Submitty and Venue.

## Database
The RCOS database is a Postgres DB running on our own infrastructure. Access is restricted to coordinators and faculty advisors, but the schema and tools used are open-sourced here.

## API

This database is served by a Postgrest API at [TODO: put api url here] that allows access to some resources when unauthenticated for public access, and allows full access to all resources when authenticated. RCOS infrastructure cannot connect to the database directly but must interact through this API.

## Setup

1. Run `pre.sql` to create database and `api` schema
2. Run `rcos.sql` to create tables and types
3. Run `post.sql` to create the roles that Postgrest needs to use
4. Run `procedures.sql` to add procedures
