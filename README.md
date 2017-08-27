# Zas

[![Build Status](https://travis-ci.org/juanibiapina/zas.svg?branch=master)](https://travis-ci.org/juanibiapina/zas)

Zas is a tool to help with local web development, inspired by [Pow](http://pow.cx).

It works by running a DNS server that resolves `.dev` domains to your local
machine. Then, when the browser makes requests to `awesome.dev`, zas proxies
the requests to your configured applications.

## Operating System Notes

Install scripts are only provided for OSX. To make it work on Linux, you need
to setup a custom dns rule (and port) that points to zas and an iptable rule to
redirect http traffic from port 80 to the zas port.

Pull requests are very welcome.

## Installation

**WARNING**: Current install instructions are broken. Zas needs to be compiled
from source.

**Zas can't run alongsize Pow, make sure you remove one before installing the other**

To install, run:

```
brew tap juanibiapina/zas
brew install zas
zas install
```

To uninstall, run:

```
zas uninstall
brew uninstall zas
```

Sudo is required for both operations in order to setup the port forwarding rules.

## Usage

### Configuring Applications

To configure an application with Zas, add a mapping on
`~/.config/zas/apps.toml` with the name of your app and the port where it is
running:

```toml
app_name = 3000
```

Make sure your app is running on the port you specified. Zas will not try to
run it in any way.

Now you can use `app_name.dev` in your browser to access the application.

### CLI

Zas comes with a CLI. Run `zas commands` for a list of possible commands.

- commands: displays a list of available commands
- help: displays help for a command
- install: setup zas system hooks
- uninstall: removes zas system hooks

## How does it work?

Zas runs a DNS server on port 12043 that resolves `.dev` domains to
`127.0.0.1` and ignores any other domains.

It also has an HTTP server running on port 12044, and the install script sets
up a rule that forwards requests on port 80 to 12044. When Zas receives the
request, it extracts the `Host` header to determine the name of the app to
proxy the request to.

If there is a port mapping for that app on the config file, zas proxies the
request to that port.

## Contributing

Clone the repository and make sure the tests run. You'll probably need:

- [Rust](https://www.rust-lang.org) stable
- [bats](https://github.com/sstephenson/bats)
- [Nodejs](https://nodejs.org)
