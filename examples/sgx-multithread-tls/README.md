# SGX Multi thread and TLS example

This is an example of a program that creates a server in SGX that can answer multiple echo requests over an encrypted TLS channel

# How to use

Run the server with

`make run`

Connect to it using TLS and send messages. OpenSSL can be used with the following command:

`make connect`

that runs

`openssl s_client -connect localhost:7878`
