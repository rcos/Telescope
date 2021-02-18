# Telescope
Telescope intends to replace [Observatory](https://github.com/rcos/observatory-server) 
as the RCOS website.

### Installation:
1. Install dependencies:
    1. Rust (see [https://www.rust-lang.org/](https://www.rust-lang.org/) for more info)
        ```shell
        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        $ source ~/.cargo/env
        ```
    2. OpenSSL and libssl (see [https://www.openssl.org/](https://www.openssl.org/) for more info)
        ```shell
        $ sudo apt update
        $ sudo apt install openssl libssl-dev libssl-doc
        ```
    3. Hasura CLI to run database migrations. See 
       [the hasura CLI docs](https://hasura.io/docs/1.0/graphql/core/hasura-cli/install-hasura-cli.html#install-hasura-cli) 
       for more info.
        ```shell
        $ curl -L https://github.com/hasura/graphql-engine/raw/stable/cli/get.sh | bash
        ```
    4. Docker and docker-compose to run telescope and the database locally. 
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
   
3. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.
4. Copy the configuration templates as follows:
    - `config_example.toml` -> `config.toml`
    - `.env.example` -> `.env`
    
    Then modify them to match your environment. You may need to generate the 
    GitHub related credentials. Go [here](https://github.com/settings/applications/new)
    to register a new GitHub OAuth application or get a new client secret.
   
5. Build and start the docker images.
    ```shell
    $ docker-compose up -d 
    ```

6. Run the database migrations. Replace the "xx.." in the command with the admin 
   secret from your `.env` file. 
    ```shell
    $ hasura --project rcos-data/ migrate --admin-secret xxxxxxxxxxxxxxxxxxxxxxxx --endpoint http://localhost:8000 apply
    ```

7. At this point Postgres, the Hasura GraphQL API, the Swagger API explorer, and 
   Telescope should all be running on your system. To shut them all down, run
   ```shell
   $ docker-compose down
   ```
   If you only want to make changes to telescope, you don't need to take down
   all the containers. Simply make your changes, run `cargo check` to verify 
   that it compiles, and then rebuild just telescope in docker using
   ```shell
   $ docker-compose up --build -d
   ```
