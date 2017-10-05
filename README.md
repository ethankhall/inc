# inc

Inc[luding] your configuration, one step at a time.

## What
`inc` is a tool that's intended to make it easier for developers to share a common set of actions across the team. These actions are up to the team to decide, but will most often be things like:
- `build`
- `test`
- `integration-test`
- `smoke-test`

But these are only examples. `inc` is intended to be flexable enought for teams to describe what they need, to do. A common usecase is to put complicated build logic behind a single command so no new team member needs to remember this project's specific parameters.

## What's Included?
While `inc` doesn't come with AA batteries, it comes with the the coin-cell to get you going. There are several commands that will come with `inc`. You're free to use one, some, all or none of them. The defult commands are:
- checkout
- exec
- env-check
- toolkit

### Checkout
`inc checkout ethankhall/etrain` will determine how to checkout that project. By defult, `inc` will attempt to checkout from github. At the moment it's not very smart and will only use what it's told to on the commandline, defaulting to github.

In the future, we want add bitbucket and VCS support with the ability to pick the 'right' one automatially. While this isn't avalible right now, support does exist for custom services. For more details on how that would work checkout the docs for [checkout](docs/checkout.md).

### Exec

> This is a work in progress, and is not finished.

The exec extension allows developers to collect the scripts, applications, flow of things that are needed for a given project and codify them.

A simple example of this is CI jobs using different providers. If you have a project that runs on Linux, Mac, and Windows you need to run CI on all of them. Each of the CI solutions may have different ways to describe how to do the build. Using `inc exec` you can put all of that logic behind a `inc exec ci` command. This will keep the custom configuration down to each CI and allow all the changes to be testing the same way.

Another common usecase is describing the acceptance tests using `inc exec`. This allows open source teams to describe whats needs locally before a merge will happen. This ensures that everyone handles a PR the same way and gives the contributer an idea of what's expected to work.

### Env-Check

> This is a work in progress, and is not finished.

The `env-check` command gives teams an easy way to validate that the machine they are using has the needed infrastructure avaliable. 

In a world where most things are Dockerized, people still do operations locally. This command validates that the tooling your project requires.

Some comon tools to validate are:
- Docker version
- Python version
- Java JDK + version

### Toolkit

> This is a work in progress, and is not finished.

The `toolkit` command is a place that I've put things that I use often enough that I don't want to have to look them up every time.

The first things that will be implemented are:
- random number
- random long
- random string
- random alpha-numberic
- random name
- uuid

These commands are intended to help teams generate test data. UUID's are hard to build by hand for test data, so having a place to easily get some common random values is nice.

This command is because I think it would be useful, not because it will be.

### Custom Commands
`inc` provides a way for an organization to implement custom commands. This would probably be done only where companies can add applications to systems.

To integrate with `inc` at the command level, you just need to have an execuable on the `PATH` and have it start with `inc-{command name}`.

More details on how this will work is coming.