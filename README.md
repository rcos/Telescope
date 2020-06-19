# Telescope
Telescope intends to replace [Observatory](https://github.com/rcos/observatory-server) 
as the RCOS website.

### Prerequisites:
1. Install rust. See [https://www.rust-lang.org/](https://www.rust-lang.org/) 
    for more details.
2. Install Postgres. See [https://www.postgresql.org/](https://www.postgresql.org/) 
    for instructions on how to do this. 
3. Clone this repository.
4. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.

### Running:
```shell script
$ # Usage:
$ cargo run -- -h

$ # Development profile (for testing on local machine).
$ cargo run -- --development

$ # Production profile (used for deploying in production environments).
$ cargo run --release -- --production
```
