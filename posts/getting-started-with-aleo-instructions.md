# Getting Started with Aleo instructions

This guide is a walkthrough on some simple examples to help you with Aleo instructions.
Instructions are like assembly for Aleo programs. Higher-level programming can be done with Leo, which you can read about [here](https://developer.aleo.org/developer/language/flying_tour).

Before we start, make sure you have Rust installed and running. You can find several examples in the Aleo's project [repository](https://github.com/AleoHQ/aleo/)

## Install Aleo

* Build from Source Code

You can install `aleo` by building from the source code as follows (we recommend installing Aleo this way):

```
# Download the source code
git clone https://github.com/AleoHQ/aleo && cd aleo

# Install Aleo
$ cargo install --path .
```

* Build from [crates.io](https://crates.io/crates/aleo)

*(This option is outdated, we recommend building from source to get the latest version)*

Alternatively, you can install aleo compiler from the crates.io repository. In your terminal, run:

```
cargo install aleo
```



At this point, you can run the `aleo` command in your terminal:

``` bash
aleo
```


## Firsts steps

### Creating a new Aleo project

To create a new project, we'll use the `new` command. Our project:


``` bash
aleo new foo
```

This will create **foo** directory and the files with the basic structure of the project:

* **README.md** having the skeleton of a README with instructions on how to compile.
* **main.aleo** the main file of the source code.
* **program.json** containing the identification of the project in JSON format. Particularly, a dev address and its private key for the program.

Let's open *main.aleo* and define the sum function:

```
// The 'foo.aleo' program.
program foo.aleo;

function sum:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
```

We will dig on what this code means in a second. First, we are going to build our foo program.

### Compiling the Aleo project

To compile the project, run in the main directory:

``` bash
aleo build
```

You will see output like this:

```
⏳ Compiling 'foo.aleo'...

 • Loaded universal setup (in 1478 ms)
 • Built 'sum' (in 6323 ms)

✅ Built 'foo.aleo' (in "[...]/foo")
```

First, a "universal setup" is loaded into your environment. You can read more about this [here](https://www.aleo.org/post/announcing-aleo-setup) or in the [Marlin paper](https://eprint.iacr.org/2019/1047.pdf).

Once the universal setup is ready, every function in your *main.aleo* file is built, generating this in the output folder:

* **sum.prover** the prover for the sum function.
* **sum.verifier** the verifier for the sum function.
* **main.avm** the bytecode of your aleo program to be run by the VM.

As you can already guess, we have only one .avm file for the whole program, but a prover and verifier for every function.

### Running a program

You can run a program with the `aleo run` command, followed by the function name you want to execute and its input parameters. Let's run our sum functions:

``` bash
aleo run sum 2u32 3u32
```

when the executing is finished, you should see the following output:

``` bash
🚀 Executing 'foo.aleo/sum'...

 • Calling 'foo.aleo/sum'...
 • Executed 'sum' (in 1170 ms)

➡️  Output

 • 5u32

✅ Executed 'foo.aleo/sum' (in "[...]/foo")
```

As you can see here, the sum function execution lasted 1170ms and the output register was assigned with the `5u32` value, representing the sum of the inputs.

### Overview of a program

Let's examine the foo program inside the *main.aleo* file:

```
// The 'foo.aleo' program.
program foo.aleo;

function sum:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
```

First, we need to declare the program as follows:

```
program foo.aleo;
```

Then, we can start writing its functions (or other Aleo structures such as interfaces, records, closures, as we will see later). In the case of functions we have it very easy:

```
function [function_name]:
```

The functions are composed of three main parts:

* **The input section**. Here we declare its input parameters:
```
    input r0 as u32.public;
    input r1 as u32.private;
```
Everything in Aleo instructions is declared/stored inside a register with a type (`i8`,`field`,`bool`, etc) and a visibility option (`public` or `private`). Registers are named as `r0`, `r1`, ..., `rn`. We will dig further into registers, types and visibility later on.

So, in this case, we use `r0`, and `r1` to store the inputs passed in sequential order to a program as `u32` values, where we can store 32-bit unsigned integers to perform our sum operation.


* **The instructions section**. The next section, consists in the core of our function. Here we call the amount of Aleo instructions we need to make our program do what we want. For example, performing an add operation:

```
    add r0 r1 into r2;
```

Every Aleo instruction is followed by its input parameters with its specifit types, and the result is stored in the *into* register. You can find all the available Aleo instructions [here](https://hackmd.io/@aleo/SJ0mrYRv5#shr).

* **The output section**. Similar to the input section, the output section does the same for the output of the program. It's the return value of the function.

```
    output r2 as u32.private;
```

### Digging into some concepts

#### Types
Aleo is a typed language. Currently, the following data types are available:

Aleo defined:
```
Boolean
Field
Group
I8
I16
I32
I64
I128
U8
U16
U32
U64
U128
Scalar
```

User defined:
```
Interface
Record
```


#### Registers

Register are the places where you store data to then be able to modify it.

#### Interfaces

Interfaces are user-defined data structures. They are very much like traditional structs in conventional programming languages. You can store Interfaces into registers, like with any other Aleo data types.

For example, let's build an interface representing a fixed-size array of 3 elements:

```
interface array3:
    a0 as u32;
    a1 as u32;
    a2 as u32;
```

Now, just for example purposes, let's code a function that adds one to every element of a register with an array3 data type stored in it.

```
function sum_one_to_array3:
    input r0 as array3.private;
    add r0.a0 1u32 into r1;
    add r0.a1 1u32 into r2;
    add r0.a2 1u32 into r3;
    cast r1 r2 r3 into r4 as array3;
    output r4 as array3.private;
```

As you can see, we can input an interface into register `r0` and access interface elements with the `.` syntax. We perform the `add` instruction on every element, storing the results in registers `r1`, `r2` and `r3` and, finally, we make use of the cast command to create a new array3 interface into `r4`.

Now, let's run it. In this case, the only new thing you need to know is that interfaces are passed to the cli in the following format:

```
"{a0: 1u32, a1: 2u32, a2: 3u32}"
```

Now we can execute the `aleo run` command:

```
run sum_one_to_array3 "{a0: 0u32, a1: 1u32, a2: 2u32}"
```

And we get the new array3 element as output:

```

🚀 Executing 'foo.aleo/sum_one_to_array3'...

 • Calling 'foo.aleo/sum_one_to_array3'...
 • Executed 'sum_one_to_array3' (in 1331 ms)

➡️  Output

 • {
  a0: 1u32,
  a1: 2u32,
  a2: 3u32
}

✅ Executed 'foo.aleo/sum_one_to_array3' (in "[...]/foo")

```

#### Records
A record is a fundamental data structure for encoding user assets and application state. They are very similar to interfaces, but they have two non-optional parameters:

```
record token:
    owner as address.private
    gates as u64.private
```

the `owner` refers to the Aleo address that owns the record and `gates` is the amount of credits that the record has to spend.

Records are important because they represent the basic Aleo structure to handle state in your application.

When running an Aleo function, only registers that belong to the application address can be passed as input registers. Otherwise, an error would be raised and the application wouldn't run.

You can find your development application address inside the *program.json* file:

```
{
    "program": "foo.aleo",
    "version": "0.0.0",
    "description": "",
    "development": {
        "private_key": "APrivateKey1zkpFsQNXJwdvjKs9bRsM91KcwJW1gW4CDtF3FJbgVBAvPds",
        "address": "aleo1x5nz5u4j50w482t5xtqc3jdwly9s8saaxlgjz0wvmuzmxv2l5q9qmypx09"
    },
    "license": "MIT"
}
```

#### Aleo State in a Nutshell
In Aleo, the state of an application is managed through records. An Aleo account can create a transaction to consume a record and produce a new record in its place. Records on Aleo are encrypted to the record owner address, ensuring that all records in Aleo are fully private.


### Your first Aleo Program: Making a transfer


Consider this program:
```
// The 'foo.aleo' program.
program foo.aleo;

record token:
    owner as address.private;
    gates as u64.private;
    amount as u64.private;

function transfer_amount:
    //  sender token record
    input r0 as token.record;
    // receiver address
    input r1 as address.private;
    // amount to transfer
    input r2 as u64.private;

    // final balance of sender
    sub r0.amount r2 into r3;
    // final balance of receiver
    add 0u64 r2 into r4;

    // sender token record after the transfer
    cast r0.owner r0.gates r3 into r5 as token.record;
    // receiver token record after the transfer
    cast r1 0u64 r4 into r6 as token.record;

    // sender new token record
    output r5 as token.record;
    // receiver new token record
    output r6 as token.record;

```
First, we defined our own record type data type called `token`, that has the two non-optional parameters, `owner` and `gates`, and a user-defined parameter called `amount`, representing the amount of tokens we have.

This `transfer_amount` function receives 3 input parameters (`sender` record, `receiver` record and `amount`) and stores them in 3 registers (`r0`, `r1` and `r2`). After that, it computes the final balance for both of them and stores it in `r3` and `r4` (using **sub** and **add** instructions to compute the subtraction and addition). With those final amounts, it creates the output records for sender and receiver, storing them in `r5` and `r6` . Finally, both records are sent out of the function with the **output** instruction.

To run this function, the first parameter is the input record of the program. The format of this parameter is the same as for interface types:


```
{
  owner: aleo1ljfzdypkggkzuvweyzat535r4kczzguyfmctwd67fm3vn6n9ggyqcx8tc7.private,
  gates: 0u64.private,
  amount: 100u64.private
}
```

where:

* owner: is the public address of the program.
* gates: are the gates that the record has.
* other parameters: depending on the program itself (in this example, we used the parameter _amount_ with the value 100).

Let's run the `transfer_amount` function:
``` bash
aleo run transfer_amount "{
owner: aleo1x5nz5u4j50w482t5xtqc3jdwly9s8saaxlgjz0wvmuzmxv2l5q9qmypx09.private,
gates: 0u64.private,
amount: 50u64.private
}" aleo1h3gu7fky36y8r7v2x9phc434fgf20g8qd7c7u45v269jfw6vmugqjegcvp 10u64
```

We get the following output records:

```

🚀 Executing 'foo.aleo/transfer_amount'...

 • Calling 'foo.aleo/transfer_amount'...
 • Executed 'transfer_amount' (in 3520 ms)

➡️  Outputs

 • {
  owner: aleo1x5nz5u4j50w482t5xtqc3jdwly9s8saaxlgjz0wvmuzmxv2l5q9qmypx09.private,
  gates: 0u64.private,
  amount: 40u64.private
  _nonce: 2293253577170800572742339369209137467208538700597121244293392265726446806023group.public
}
 • {
  owner: aleo1h3gu7fky36y8r7v2x9phc434fgf20g8qd7c7u45v269jfw6vmugqjegcvp.private,
  gates: 0u64.private,
  amount: 10u64.private
  _nonce: 2323253577170856894742339369235137467208538700597121244293392765726742543235group.public
}

✅ Executed 'foo.aleo/transfer_amount' (in "[...]/foo")

```

And that's it. You had transferred your first own-defined tokens in Aleo!

Note: the _nonce is not written in Aleo instructions. The compiler outputs the _nonce in record outputs. The user needs to provide in as input when using a record.


### Final remarks

Aleo programs can compile and execute _cryptographic circuits_ written in Aleo instructions. This introduces some restrictions that don't exist in conventional programming languages. For example, when you use the _ternary_ operator to have a branch in the program flow, you must have both results in advance (i.e. the result of each branch must be calculated and known before executing the ternary operator). A boolean comparison must be between registers, and not between the return values of function calls.
