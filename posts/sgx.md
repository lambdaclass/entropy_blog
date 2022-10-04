# SGX
## Introduction

Intel SGX is a set of instructions used to enhance the security of application code and data, which gives the users a great level of protection. Intel SGX helps to keep the users’ sensitive data from being accessed by creating a trusted execution environment within memory..

Sensitive data may be medical records, financial assets, passwords, encryption keys, biometric identification. In summary, any information that could cause harm if it's revealed.

To abstract from operating system security vulnerabilities that host devices may have, essential parts of the project are deployed in a specific trusted execution environment (TEEs).

TEEs can be thought of as processes that run "isolated" from the operating system and have higher layers in a safe part of the CPU. TEEs are intended to ensure the integrity of data, code, and data privacy.

Intel SGX is an Intel Instruction Set Architecture (ISA) extension with TEEs support. The environments are called enclaves.

Some important items:

* **It's not possible to read nor write the enclave's memory space from outside the enclave**, regardless of the privilege level and CPU mode.
* In production, it's not possible to debug enclaves by software or hardware..
* Entering the enclave via function calls, jumps or stack/register manipulation is not possible. To do so you have to use a specific CPU instruction which also does some safety checks.
* **Enclave's memory is encrypted**, and the key used changes on every power cycle. It's stored within the CPU and is not accessible.

![](https://i.imgur.com/eUXGLjZ.png)


## SGX's security features: remote attestation and encryption.

While SGX provides a safe enclave to run applications in encrypted parts of the memory that can't be acceded from outside the enclave, there are additional issues to consider when communicating the enclave with external applications.

The first one that has to be addressed is the authenticity of the enclave. This is the same as a website using a certificate to prove its authenticity, but in the case of an enclave, we can't use a certificate. The process to prove the authenticity to a party in another machine is called **Remote Attestation**.

Another issue is the Secret Provisioning to the remote enclave. This is usually done by establishing a safe channel with the enclave, and depending on the implementation, the method may vary.

The last issue that we are not addressing in this document is called **Sealing**, that's the persistence of data, using encryption so only the enclave can read it.


### Attributes to consider when selecting how to implement SGX enclave.



| Library                              | Language | Developer                               | On-premise or not (ECDSA or EPID) | Relay secrets through unsafe proxy server difficulty | Currently Supported | Pros                                                                                               | Cons                                                                                                                                                                                                                            | URLs                                                                                  |
| ------------------------------------ | -------- | --------------------------------------- | --------------------------------- | ---------------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Manual Implementation                | ---      | ---                                     | Yes                               | Low                                                  | -                   | - We can adapt it to our requirements <br> - Intel's RA documentation to follow is really detailed | - Everything has to be done from scratch, so there will be more work <br> - Code will need to be heavily audited, and improved, for it to be safe <br> - Encryption schemes for secrets are not provided by Intel documentation | Some examples of similar code: https://github.com/ndokmai/rust-sgx-remote-attestation |
| Linux SGX Trust Management Framework | C++      | IBM                                     | No                                | Easy                                                 | No                  | - Design fits our use case <br> - Secret provisioning is easy                                      | - No ECDSA <br> - It's not updated to work with current SGX version                                                                                                                                                             | https://github.com/IBM/sgx-trust-management                                           |
| RA-TLS                               | C++      | Intel Labs Cloud Security Research      | Yes                               | Medium                                               | Yes                 | - Intel project <br> - Abstracts the attestation process                                           | Requires the usage of the library on both ends <br>                                                                                                                                                 | https://github.com/intel/sgx-ra-sample                                                |
| Gramine                              | -        | Gramine <br> (with intel contributions) | Yes                               | No                                                   | Yes                 | - Run any application with SGX                                                                     | Everything has to be used with gramine                                                                                                                                                                                          | https://gramine.readthedocs.io/en/latest/                                             |



Before deciding on an implementation we must answer some questions such as:

* Do we need Remote Attestation, or do we assume the communication is being made with an enclave?
* Unsafe proxy server may not be needed if secrets are provisioned directly to the enclave. Do we pass the secrets through the proxy server?
* Remote attestation can be costly to use in every connection. Should we try to reduce this overhead?



### [Fortanix-SGX](https://docs.rs/fortanix-sgx-abi/latest/fortanix_sgx_abi/)
The Fortanix SGX ABI (compiler target x86_64-fortanix-unknown-sgx) is an interface for Intel SGX enclaves. It is a small yet functional interface suitable for writing larger enclaves. In contrast to other enclave interfaces, this interface is primarily designed for running entire applications in an enclave.
It applies the microcode updates supplied by Intel® and disables hyperthreading on all systems. This prevents unauthorized access to the memory of SGX enclaves through side-channel attacks such as the Foreshadow vulnerability.

#### Examples.
##### Example 1. Program that spawns multiple threads to make a computation using one SGX Vault
https://github.com/Entropy1729/blog/tree/main/examples/sgx/sgx-multithread
To run the program:
```
cargo run --release --target x86_64-fortanix-unknown-sgx
```
or 
```
make run
```
The quantity of threads is defined in `Cargo.toml` file in the section `[package. metadata. fortanix-sgx]`. In this case, there are defined 5 threads.
The limit of the threads depends on the available memory. If this limit is exceeded, the enclave closes.
In addition, it's not possible to join threads inside the enclave.

This image shows the execution of 5 threads inside the enclave.
![](https://i.imgur.com/Bjpf96y.png)


##### Example 2. SGX server that answers multiple echo requests over an encrypted TLS channel
https://github.com/Entropy1729/blog/tree/main/examples/sgx/sgx-multithread-tls
To run the program:
```
cargo run --release --target x86_64-fortanix-unknown-sgx
```

The way to communicate with an enclave is through the use of the TCP protocol.
In this example, a calculation is not made in each thread, but it is demonstrated how it connects through TCP with the enclave.
The TLS protocol is used for data encryption purposes between the enclave and an app that wants to communicate with it.
The problem with TLS is that the certificate does not guarantee that the communication is being made with an enclave. This issue is resolved by using remote attestation.

One thing to keep in mind is that the `mbedtls` library has libraries that are not thread safe.

The following image shows the use of openssl to connect with TLS.
![](https://i.imgur.com/7ziIm1w.png)

On this screen is displayed an Echo Server with 2 clients connected.
![](https://i.imgur.com/dCcb8HX.png)


### Linux SGX [Trust Management Framework](https://github.com/IBM/sgx-trust-management)
Trust Management Framework (also called TruCE - "Trust in Cloud Enclaves") handles all aspects of remote attestation and secret delivery process in Intel SGX enclaves. The framework enables application developers to focus on the application code, performing attestation by a simple API call. 

The trust Management Framework is a service model that can have different implementations underneath, but the advantage is that it always shows the same interface to the applications that invoke the service..

The difference with this implementation of remote attestation is that the generation of an initial secret is made inside the enclave instead of sending it to the enclave.

It generates an RSA private/public key pair and embeds the public key (hash) in the enclave attestation quote.

Since the quote is signed by Intel keys, It allows the benefit of keeping the resulting attestation report in an untrusted storage. This brings the chance of reducing the trust requirements placed on the TrueCE server

The application clients can verify an enclave report by using the Intel attestation public key, retrieve the enclave public key and use it for the encryption of secrets (such as data keys) to be sent to the enclave for subsequent decryption and processing of sensitive data.

Trust Management Framework consists of two main components:

```mermaid
  flowchart LR
   TustCE --> TruCE_Server & TruCE_SDK 
   ```
  
TruCE server: Standalone process that registers with Intel Attestation Service and assists in remote attestation of RestAssured platform enclaves.

TruCE SDK: Toolkit for application development. It has API and libraries for the trusted (enclave) part of the cloud application, the untrusted part of the cloud application, and the off-cloud client code that interacts with the cloud application.

Trust Management Framework can run in a simulated IAS mode so TruCE doesn't need registration with Intel. It doesn't contact the IAS and skips the attestation report signature verification step..

### [RA-TLS](https://github.com/cloud-security-research/sgx-ra-tls)
The project provides a ***proof-of-concept implementation*** on how to integrate Intel SGX remote attestation into the TLS connection setup. It extends the standard X.509 certificate with SGX-related information

These are the instructions to start SGX Server.
First of all, clone the github repository using `git clone github.com/cloud-security-research/sgx-ra-tls.git` into the folder `sgx-ra-tls`
Build the docker file `Dockerfile-ubuntu18.04` with parameters `-t ratls `.
*Note:* The following command needs root privileges to be able to use /dev/isgx

To run the docker file: `sudo docker run --device=/dev/isgx --privileged=true -v /var/run/aesmd:/var/run/aesmd -v$(pwd):/project -it ratls bash`

Into the folder project, run `SPID=your_spid EPID_SUBSCRIPTION_KEY=your_epid_subscription_key QUOTE_TYPE=SGX_LINKABLE_SIGNATURE bash ra_tls_options.c.sh > ra_tls_options.c`
Finally to start the Server, `./build.sh sgxsdk ` and `( cd deps/wolfssl-examples/SGX_Linux ; ./App -s )`

To run the Client use the same docker file, but previously get the container Id with docker ps.
Using this ID, run `docker exec -ti --user root [container id] bash.`
execute `./project/wolfssl-client .` to start the client
This screen shows the execution
![](https://i.imgur.com/FhkJ9fn.png)

### [Gramine](https://gramineproject.io/)
Gramine is a lightweight guest OS, designed to run a single Linux application with minimal host requirements. Gramine can run applications in an isolated environment with benefits comparable to running a complete OS in a virtual machine – including guest customization, ease of porting to a different host OS, and process migration.on.

Gramine supports running Linux applications using the Intel SGX (Software Guard Extensions) technology (sometimes this version is called Gramine-SGX). With Intel SGX, applications are secured in hardware-encrypted memory regions (called SGX enclaves). SGX protects code and data in the enclave against privileged software attacks and against physical attacks on the hardware off the CPU package (e.g., cold-boot attacks on RAM). Gramine can run unmodified applications inside SGX enclaves, without the toll of manually porting the application to the SGX environment..

## Appendix

### Relay secrets through unsafe proxy server

In the initial architecture design, we added a server between the user and the enclave. This server delegates tasks to the enclave.

This design adds the complexity of passing the secrets through the unsafe server, which may be more or less complex depending on the approach.

Some approaches, like having Gramine, don't allow us to do this. Others, like RA TLS, require additional work to do this..

Alternatives for this design include letting the user provision the secrets directly to the enclave or running the whole application inside it.

### ECDSA vs EPID

The remote attestation can be done in two different ways, one is [EPID attestation](https://api.portal.trustedservices.intel.com/EPID-attestation), and the other [ECDSA attestation](https://www.intel.com/content/www/us/en/developer/articles/technical/quote-verification-attestation-with-intel-sgx-dcap.html). EPID uses an Intel attestation server, an Intel service, and ECDSA lets you build your own attestation infrastructure.
