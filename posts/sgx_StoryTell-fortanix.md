# How to implement a working Software Guard Extensions (SGX) project

## What is SGX?

Intel SGX is a set of instructions used to enhance the security of application code and data, which gives the users a great level of protection. Intel SGX helps to keep the users’ sensitive data from being accessed by creating a trusted execution environment within memory.

In other words, SGX encrypts sections of memory using security instructions native to the CPU. It’s a way of hardware-based encryption that allows users to protect their most sensitive data by placing it in a highly secure environment.

Many industries can take advantage of the SGX, it's not only for IT Apps. Sensitive data may be medical records, finance and insurance assets, passwords, encryption keys, biometric identification, and the Military industry. In summary, any information that could cause harm if it's revealed.

To abstract from operating system security vulnerabilities that host devices may have, essential parts of the project are deployed in a specific trusted execution environment (TEEs).

TEEs can be thought of as processes that run "isolated" from the operating system and have higher layers in a safe part of the CPU. TEEs are intended to ensure the integrity of data, code, and data privacy.

Intel SGX is an Intel Instruction Set Architecture (ISA) extension with TEEs support. The environments are called enclaves.

The original goal was to secure the remote computation, it´s explained in this [paper](https://eprint.iacr.org/2016/086.pdf) published by MIT’s Computer Science and Artificial Intelligence Laboratory. 

## What is an SGX Enclave?

Enclaves are memory zones that are isolated and contain sensitive, protected application data.
The code and data in these memory sectors are only accessible within the enclave.
When an application is run inside an enclave, the CPU instantly encrypts it and stores the key there.
Because the key is protected within the CPU, a malicious user cannot obtain it by inspecting system memory.
![](https://i.imgur.com/eUXGLjZ.png)


## Which Intel CPUs use SGX?
To verify which Intel CPUs use SGX, please refer to the [Intel product search page](https://ark.intel.com/content/www/us/en/ark/search/featurefilter.html?productType=873).
There is a Filter drop-down menu. Scroll down and select Intel® Software Guard Extensions (Intel® SGX).

![](https://i.imgur.com/RG4A7I5.png)


## Can any application run in an SGX Enclave?
To answer this question there are 2 options. They are presented by [Intel](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/get-started.html).
### Build your application.
To build an application to run SGX, many pieces of code have to be rewritten in the source code.
These parts have to be linked with the [Intel’s SGX SDK](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/linux-overview.html) to make it SGX ready.
### Secure Your Existing Application
There are many *open-source LibOS projects* that support Intel SGX.
The most popular is [Gramine](https://gramineproject.io/).
It is a lightweight guest OS, designed to run a single Linux application with minimal host requirements. Gramine can run applications in an isolated environment with benefits comparable to running a complete OS in a virtual machine – including guest customization, ease of porting to a different host OS, and process migration.on.

Gramine supports running Linux applications using the Intel SGX (Software Guard Extensions) technology (sometimes this version is called Gramine-SGX). With Intel SGX, applications are secured in hardware-encrypted memory regions (called SGX enclaves). SGX protects code and data in the enclave against privileged software attacks and against physical attacks on the hardware off the CPU package (e.g., cold-boot attacks on RAM). Gramine can run unmodified applications inside SGX enclaves, without the toll of manually porting the application to the SGX environment.

Also, there are companies that offer *commercial LibOS solutions* that support Intel SGX, for example [Fortanix](https://www.fortanix.com/).
The Fortanix SGX ABI (compiler target x86_64-fortanix-unknown-sgx) is an interface for Intel SGX enclaves. It is a small yet functional interface suitable for writing larger enclaves. In contrast to other enclave interfaces, this interface is primarily designed for running entire applications in an enclave.

It applies the microcode updates supplied by Intel® and disables hyperthreading on all systems. This prevents unauthorized access to the memory of SGX enclaves through side-channel attacks such as the Foreshadow vulnerability.

## How is the interaction between the App and the Enclave?
To illustrate how the interaction between an application and the enclave is, [Intel](https://www.intel.com/content/www/us/en/developer/articles/technical/sgx-intro-passing-data-between-app-and-enclave.html) presents a basic tutorial in C++ that we will follow next.

### Into the Enclave:
The enclave functions will be declared based on the following EDL file:
```
enclave {
   trusted {
      //ECALLS
      public void enclaveInFunction([in, size=len] char *buf, size_t len);
      public void enclaveOutFunction([out, size=len] char *buf, size_t len);
      public void enclaveInOutFunction([in, out, size=len] char *buf, size_t len);
   };
};
```

**enclaveInFunction()**

This function demonstrates the use of both an 'in' and 'out' enclave by swapping the values of the input string and the internal enclave string. Data is exchanged between the application and the enclave.

**enclaveOutFunction()**

This function demonstrates the use of an 'out' enclave by changing the value of an externally provided input parameter. Data is sent from the enclave to the application

**enclaveInOutFunction()**

This function demonstrates the use of both an 'in' and 'out' enclave by swapping the values of the input string and the internal enclave string. Data is exchanged between the application and enclave.

#### Enclave Code:

```
#include "Enclave1_t.h"
#include "sgx_trts.h"
#include <stdlib.h>
#include <string.h>

#define MAX_BUF_LEN 100
char enclaveString[MAX_BUF_LEN] = "Internal enclave string is not initialized";


void enclaveOutFunction(char *buf, size_t len)
{
        if(len < MAX_BUF_LEN)
                buf = (char*)malloc(MAX_BUF_LEN);
        memcpy(buf,enclaveString,strlen(enclaveString)+1);
}

void enclaveInFunction(char *buf, size_t len)
{
        if(len <= (size_t)MAX_BUF_LEN)
                memcpy(enclaveString,buf,strlen(buf)+1);
}

void enclaveInOutFunction(char *buf, size_t len)
{
        char *tmp = (char*)malloc(MAX_BUF_LEN*sizeof(char));
        memcpy(tmp,buf,strlen(buf)+1);
        memcpy(buf,enclaveString,strlen(enclaveString)+1);
        memcpy(enclaveString,tmp,strlen(tmp)+1);
        free(tmp);
}

```
### Application Code:

SGX enclave code has no console output. To test the enclave function, a console application must be created to call the enclave functions.

```
#include "stdafx.h"
#include "sgx_urts.h"
#include "Enclave1_u.h"
#include <stdio.h>
#include "sgx_capable.h"
#include "sgx_uae_service.h"

#define ENCLAVE_FILE _T("Enclave1.signed.dll")
#define MAX_BUF_LEN 100


int main()
{
        sgx_enclave_id_t enclaveId = NULL;
        sgx_status_t ret = SGX_SUCCESS;
        sgx_launch_token_t token = {0};
        sgx_launch_token_t *launchToken = NULL;
        int updated, i=0;
        char buffer[MAX_BUF_LEN] = "Initial string, before enclave calls";

        if(sgx_is_capable(&updated) != SGX_ENABLED)
        {
                printf("Error %#x: SGX is not enabled on this device\n", ret);
                return -1;
        }

        printf("%i: %s\n", i++, buffer);

        ret = sgx_create_enclave(ENCLAVE_FILE, SGX_DEBUG_FLAG, &token, &updated,
                &enclaveId, NULL);
        if(ret != SGX_SUCCESS)
        {
                printf("Error %#x: cannot create enclave\n", ret);
                return -1;
        }

        enclaveOutFunction(enclaveId, buffer, MAX_BUF_LEN);
        printf("%i: %s\n", i++, buffer);

        //set the internal enclave function
        strcpy_s(buffer,"Changed the enclave string");
        enclaveInFunction(enclaveId, buffer, MAX_BUF_LEN);

        //swap values with enclave string
        strcpy_s(buffer,"New value application string");
        enclaveInOutFunction(enclaveId, buffer, MAX_BUF_LEN);

        //now, buffer should be "Changed the enclave string"
        printf("%i: %s\n", i++, buffer);

        //swap again; next output should be "New value for application string"
        enclaveInOutFunction(enclaveId, buffer, MAX_BUF_LEN);
        printf("%i: %s\n", i++, buffer);

        //grab the pre-swapped string "Changed the enclave string"
        enclaveOutFunction(enclaveId, buffer, MAX_BUF_LEN);
        printf("%i: %s\n", i++, buffer);


        if(sgx_destroy_enclave(enclaveId) != SGX_SUCCESS)
        {
                printf("Error %x: cant destroy enclave\n", ret);
                return -1;
        }
        else printf("DONE\n");
        getchar();

        return 0;
}

```
### Output:
```
                0: Initial string, before enclave calls
                1: Internal enclave string is not initialized
                2: Changed the enclave string
                3: New value application string
                4: Changed the enclave string
                DONE
```

#### [Fortanix Program example that spawns multiple threads to make a computation using one SGX Vault](https://github.com/Entropy1729/blog/tree/main/examples/sgx/sgx-multithread)
To run the program:
```
cargo run --release --target x86_64-fortanix-unknown-sgx
```
or 
```
make run
```
The quantity of threads is defined in `Cargo.toml` file in the section `[package. metadata. fortanix-sgx]`. In this case there are defined 5 threads.
The limit of the threads depends on the available memory. If this limit is exceeded, the enclave close.
In addition, it's not possible to join threads inside the enclave.

This image shows the execution of 5 threads inside the enclave.
![](https://i.imgur.com/Bjpf96y.png)

## SGX's security features: remote attestation and encryption.

While SGX provides a safe enclave to run applications in encrypted parts of the memory that can't be acceded from outside the enclave, there are additional issues to consider when communicating the enclave with external applications.

The first one that has to be addressed is the authenticity of the enclave. This is the same as a website using a certificate to prove its authenticity, but in the case of an enclave, we can't use a certificate. The process to prove the authenticity to a party in another machine is called **Remote Attestation**.

Another issue is the Secret Provisioning to the remote enclave. This is usually done by establishing a safe channel with the enclave, and depending on the implementation, the method may vary.

The remote attestation can be done in two different ways, one is [EPID attestation](https://api.portal.trustedservices.intel.com/EPID-attestation), and the other [ECDSA attestation](https://www.intel.com/content/www/us/en/developer/articles/technical/quote-verification-attestation-with-intel-sgx-dcap.html). EPID uses an Intel attestation server, an Intel service, and ECDSA lets you build your own attestation infrastructure.

## Wrapping up
Intel SGX was designed both to protect the hardware and to protect the software from attacks.
In hardware protection, the main feature is that the memory encryption key randomly changes every power cycle (for example, boot, sleep, or hibernate). The key is stored within the CPU and is not accessible.
Regarding software protection, the enclave environment cannot be entered via classic function calls, jumps, register manipulation, or stack manipulation. The only way to call an enclave function is via a new instruction that performs several protective checks.
As you can see there are many pros to the use of SGX.
