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

### Installation Debugging:
If you have issues running the website, here are a few things you can check/try:
1. All prerequisites are installed.
2. If you use Windows Subsystem for Linux, make sure that it's installed correctly. There have recently been issues with the installation of WSL2 and Rust: https://github.com/rust-lang/rustup/issues/2293
3. One you compile the website, launch it at the local IP address that your terminal generates. Make sure that 'https://' precedes the IP address.
4. If you get a blank page when launching the website, you need to regenerate your TLS/SSL certificate and keys. If that doesn't work, then switch browsers (e.g. Firefox, Chrome).
