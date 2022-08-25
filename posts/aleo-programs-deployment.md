# Deploying Aleo Programs in Testnet3

Aleo empowers you to write and build private applications easily.

As you read in the title, this guide is a walkthrough on how to deploy Aleo programs in Testnet3, this will be achieved using the Aleo SDK. We hope this blog post will help you to achieve this and discover all the potential of private applications.

Before you can start deploying Aleo programs you must have the Aleo SDK installed and must have an Aleo Program created.

You can follow the steps detailed in [our previous post](https://www.entropy1729.com/aleo-development-starter-pack/).

## Deploying our Aleo project

As said above, before deploying a program you must have a program to deploy. After creating your program, deploying it is very straightforward, just do the following:


```bash
cd foo

aleo deploy
```

When you have done this you should see the following output:

```bash
⏳ Deploying 'foo.aleo'...

 • Loaded universal setup (in 2174 ms)
 • Built 'hello' (in 12970 ms)
 • Certified 'hello': 657 ms

✅ Deployed 'foo.aleo' (in "[...]/foo")
```

If you run `aleo deploy` outside your program directory (lets say inside `/not_foo`) the latter is not going to be deployed and the console output should log:

```bash
⚠️  Missing 'program.json' at '[...]/not_foo'
```

And that is how you deploy an Aleo project. Easy right? That's the idea 😉.

<!--
## Checking your program was successfully deployed

TODO
-->

## Working with Aleo Programs in Testnet3

During Testnet3's life cycle all program deployments will be sent to the blockchain by Aleo Explorer on behalf of the user so no user credit spent is needed!

This will help speed up the development and get helpful metrics for this testnet.

