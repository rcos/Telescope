# Telescope - <https://rcos.io>
Telescope is the RCOS website. 

[![Cargo](https://github.com/rcos/Telescope/actions/workflows/cargo.yml/badge.svg)](https://github.com/rcos/Telescope/actions/workflows/cargo.yml)
[![Docker](https://github.com/rcos/Telescope/actions/workflows/docker.yml/badge.svg)](https://github.com/rcos/Telescope/actions/workflows/docker.yml)
[![GitHub release version](https://img.shields.io/github/release/rcos/Telescope.svg)](https://github.com/rcos/Telescope/releases)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/rcos/Telescope.svg)](https://isitmaintained.com/project/rcos/Telescope)
[![Percentage of issues still open](http://isitmaintained.com/badge/open/rcos/Telescope.svg)](https://isitmaintained.com/project/rcos/Telescope)

## User Notes
If you find issues with Telescope or have a feature ou want added, please make an issue under the issues tab. You should also feel free to contribute your own. Pull requests are welcome. See below for detailed information on building & contributing to telescope.

## Development Notes
These are note for Telescope Developers on how to find and update Telescope 
itself.

#### Project Structure
Telescope is a large enough project that it may not be immediately obvious where
certain files are. This section provides a map to this repository.
- `.github`: This folder holds configuration files related to this repository's 
    interactions with Github.com. This includes the GitHub issue templates, the 
    continuous integration workflows, and the Dependabot configuration.
- `proposals`: This folder contains the project proposal files that Telescope has
    been submitted under for the Rensselaer Center for Open Source (RCOS).
- `rcos-data`: This git submodule points to the current telescope version of the
    repository that contains the migrations for the central RCOS database.  
- `graphql`: This folder contains the introspected `schema.json` file for the 
    central RCOS GraphQL API exposed via Hasura over the central RCOS database.
    This folder also contains GraphQL files for all of the different queries
    that Telescope will send to the central API.
- `static`: This folder contains statically served files and assets, including 
    - Telescope icons
    - RCOS icons and branding
    - The global CSS style file
    - All of Telescopes javascript
    - Sponsor logos and branding
- `templates`: This folder contains all of the Handlebars templates used to 
    render Telescope's frontend. 
- `src`: This is the main Telescope codebase, written in Rust.

#### Schema Introspection
When the central RCOS GraphQL API (a Hasura wrapper over the central RCOS Postgres database) 
gets updated, Telescopes schema needs to get updated to match. After merging whatever changes
or migrations have been made to the `telescope-dev-version` branch of the `rcos-data` repository,
update Telescope's git `rcos-data` submodule to point to the newest commit on the 
`telescope-dev-version` branch. After you have done this and pulled the submodule,
update the local database using the hasura client. The commands should look like this:
```shell
$ hasura --project rcos-data/ migrate --admin-secret xxxxxxxxxxxxxxxxxxxxxxxx --endpoint http://localhost:8000 apply
$ hasura --project rcos-data/ metadata --admin-secret xxxxxxxxxxxxxxxxxxxxxxxx --endpoint http://localhost:8000 reload
``` 
where `xxxxxxxxxxxxxxxxxxxxxxxx` is replaced by the hasura admin secret in your `.env` file.
After applying the migrations, go to the hasura console to make sure that all the proper
tables are tracked, and all the types and queries are available. 
To go to the hasura console on your local machine, navigate to 
[http://localhost:8000/console/settings/metadata-actions](http://localhost:8000/console/settings/metadata-actions)
in your browser. 

If you haven't already, you should install a GraphQL client to introspect the schema. 
There are several of these that are probably acceptable, but for consistency we use 
the [`graphql-rust client`](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli). 
Install this using the command from its README.
```shell
$ cargo install graphql_client_cli --force
```
Finally, regenerate Telescope's `schema.json` file as follows:
```shell
$ graphql-client introspect-schema --header 'x-hasura-admin-secret: xxxxxxxxxxxxxxxxxxxxxxxx' --output graphql/rcos/schema.json http://localhost:8000/v1/graphql
```
again, `xxxxxxxxxxxxxxxxxxxxxxxx` is replaced by the hasura admin secret in your `.env` file.

You may also have to introspect the GitHub V4 API schema, since we also keep a 
copy of that in telescope. This requires a GitHub Personal Access Token (PAT) 
which you can generate [here](https://github.com/settings/tokens). Once you have
generated your PAT, you can introspect/update the local GitHub Schema using
```shell 
$ graphql-client introspect-schema --output graphql/github/schema.json --authorization "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" https://api.github.com/graphql
```
where `xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` is replaced by your PAT.

## Installation:
1. Install dependencies:
    1. Rust (see [https://www.rust-lang.org/](https://www.rust-lang.org/) for more info)
        ```shell
        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        $ source ~/.cargo/env
        ```
    2. Hasura CLI to run database migrations. See 
       [the hasura CLI docs](https://hasura.io/docs/1.0/graphql/core/hasura-cli/install-hasura-cli.html#install-hasura-cli) 
       for more info.
        ```shell
        $ curl -L https://github.com/hasura/graphql-engine/raw/stable/cli/get.sh | bash
        ```
    3. Docker and docker-compose to run telescope and the database locally. 
       this can be a complicated process, but there are good instructions online 
       [here](https://docs.docker.com/get-docker/).
       Message me for help if you need it.
       
2. Clone this repository:
    ```shell script
    $ git clone --recurse-submodules https://github.com/rcos/Telescope.git
    ```
   You need to make sure you get all of the submodules here using 
   `--recurse-submodules` otherwise you won't have any of the RCOS branding
   logos or icons, or any of the database migrations and setup.

4. Copy the configuration templates as follows:
    - `config_example.toml` -> `config.toml`
    - `.env.example` -> `.env`
    (note: the `config.toml` and `.env` files must be created yourself.) 
    
    Then modify them to match your environment. You may need to generate the 
    GitHub related credentials. Go [here](https://github.com/settings/applications/new)
    to register a new GitHub OAuth application or get a new client secret.
    You will also have to create a discord OAuth app and bot token. Instructions
    can be found in `config_example.toml`.  
   
5. Build and start the docker images.
    ```shell
    $ docker-compose up -d 
    ```

6. Run the database migrations. Replace the "xx.." in the command with the admin 
   secret from your `.env` file. Make sure the `admin-secret` is at least 32 characters long.
    ```shell
    $ hasura --project rcos-data/ migrate --admin-secret xxxxxxxxxxxxxxxxxxxxxxxx --endpoint http://localhost:8000 apply
    ``` 
7. Track the Hasura tables:
    In Hasura (http://localhost:8000), enter your `admin-secret` when prompted. Navigate to the "Data" tab, and click "Create Table". Track all tables, and hit "Add Table". You also want to press "Track All" for the foreign key relationships as well.
8. Reload metadata:
   ```shell
   $ hasura --project rcos-data/ metadata --admin-secret xxxxxxxxxxxxxxxxxxxxxxxx --endpoint http://localhost:8000 reload
   ```
9. At this point Postgres, the Hasura GraphQL API, Caddy, and Telescope should 
   all be running on your system in individual docker containers. Docker 
   exposes the Hasura console at http://localhost:8000 and https://localhost:8001, 
   and Telescope is exposed at https://localhost:8443. To shut them all down, run
   ```shell
   $ docker-compose down
   ```
   If you only want to make changes to telescope, you don't need to take down
   all the containers. Simply make your changes, run `cargo check` to verify 
   that it compiles, and then rebuild just telescope in docker using
   ```shell
   $ docker-compose up --build -d
   ```
