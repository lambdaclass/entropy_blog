# Deploying Aleo Programs in Testnet3

This guide is a walkthrough on how to deploy Aleo programs in Testnet3 using the Aleo SDK.

Before you can start deploying Aleo programs you must have the Aleo SDK installed and must have an Aleo Program created and built.

You can follow the steps detailed in [our previous post](https://www.entropy1729.com/aleo-development-starter-pack/).

## Deploying our Aleo project

To deploy a program we first need to have a program to deploy:

```bash
aleo new foo
```

After creating your program, you need to build it to deploy it:

```bash
cd foo

aleo build

aleo deploy
```

When you have done this you should see the following output:

```bash
⏳ Compiling 'foo.aleo'...

 • Loaded universal setup (in 1713 ms)
 • Built 'hello' (in 7172 ms)

✅ Built 'foo.aleo' (in "[...]/foo")

⏳ Deploying 'foo.aleo'...

 • Loaded universal setup (in 2174 ms)
 • Built 'hello' (in 12970 ms)
 • Certified 'hello': 657 ms

✅ Deployed 'foo.aleo' (in "[...]/foo")
```

If you run `aleo deploy` outside your program directory (let's say inside `/not_foo`) the latter is not going to be deployed and the console output should log:

```bash
⚠️  Missing 'program.json' at '[...]/not_foo'
```

And that is how you deploy an Aleo project. Easy right? That's the idea 😉.

## Working with Aleo Programs in Testnet3

For the initial Testnet3 rollout the deploy command will send deploys through the Aleo Explorer and the fees will be covered by the aleo node. This will help to get useful metrics and speed up the development of this testnet!

In the following posts, we will show you how to execute your deployed Aleo programs.
