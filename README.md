# Telescope
Telescope intends to replace [Observatory](https://github.com/rcos/observatory-server) 
as the RCOS website.

### Prerequisites:
1. Install dependencies:
    1. Rust (see [https://www.rust-lang.org/](https://www.rust-lang.org/))
        ```shell script
        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        $ source ~/.cargo/env
        ```
    2. OpenSSL and libssl (see [https://www.openssl.org/](https://www.openssl.org/) for more info)
        ```shell script
        $ sudo apt update
        $ sudo apt install openssl libssl-dev libssl-doc
        ```
    3. DbMate to run database migrations. See [https://github.com/amacneil/dbmate](https://github.com/amacneil/dbmate) for more info.
        ```shell script
        $ sudo curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64
        $ sudo chmod +x /usr/local/bin/dbmate
        ```
       
2. Make sure that you have a user and password set up in postgres and can log in
    with a password. You may have to modify `/etc/postgresql/12/main/pg_hba.conf` 
    (or something along those lines) to use md5 authentication rather than peer 
    authentication. You will know this works when you can log in to postgres 
    using
    ```shell script
    $ psql -U <username> -W
    ```
   
3. Clone this repository:
    ```shell script
    $ git clone --recurse-submodules https://github.com/rcos/Telescope.git
    ```
   You need to make sure you get all of the submodules here using 
   `--recurse-submodules` otherwise you won't have any of the RCOS branding
   logos or icons, or any of the database migrations and setup.
   
4. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.
   
5. Copy the configuration templates as follows:
    - `config_example.toml` -> `config.toml`
    - `.env.example` -> `.env`
    
    Then modify them to match your environment.
   
6. Run the database setup and migrations. This will create a database and then 
    run all of the necessary migrations. If you have stored the database url in 
    an environment variable or in your `.env` file the commands are as such:
    ```shell script
    $ diesel setup
    $ diesel migration run
    ```
   otherwise use:
    ```shell script
    $ diesel setup --database-url postgres://username:password@localhost/telescope
    $ diesel migration run --database-url postgres://username:password@localhost/telescope
    ```

### Running:
```shell script
$ # Usage:
$ cargo run -- -h

$ # Development profile (if specified in config.toml).
$ cargo run -- -p dev

$ # Production profile (if specified in config.toml).
$ cargo run --release -- -p production
```

### Installation Debugging:
If you have issues running the website, here are a few things you can check/try:
1. All prerequisites are installed.
2. If you use Windows Subsystem for Linux, make sure that it's installed correctly. 
    There have recently been issues with the installation of WSL2 and Rust: 
    https://github.com/rust-lang/rustup/issues/2293
3. One you compile the website, launch it at the local IP address that your 
    terminal generates. Make sure that 'https://' precedes the IP address.
4. If you get a blank page when launching the website, you need to regenerate 
    your TLS/SSL certificate and keys. If that doesn't work, then switch 
    browsers (e.g. Firefox, Chrome).
5. If your Postgres server isn't locally running, enter the 
    command `sudo service postgresql status`. If the service is down, 
    enter `sudo service postgresql start`.
6. If you are having issues creating a DATABASE_URL, try using the default 
    admin username 'postgres'.
