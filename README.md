# Zas

[![Build Status](https://travis-ci.org/juanibiapina/zas.svg?branch=master)](https://travis-ci.org/juanibiapina/zas)

Zas is a tool to help with local web development, inspired by [Pow](http://pow.cx).

It works by running a DNS server that resolves `.dev` domains to your local
machine. Then, when the browser makes requests to `awesome.dev`, zas brings up
your `awesome` application and forwards requests to it.

To configure an application with zas, make sure it has a `Procfile` that runs a
server on a `$PORT` variable and create a symlink to it:

```
cd ~/.zas/apps
ln -s ~/projects/awesome awesome
```

The name of the symlink (awesome) determines the hostname (awesome.dev) that
you use to access the application.

## Operating System Notes

Currently only OSX is supported.

To make it work on Linux, there needs to be an easy way to setup a custom dns
rule (and port) that points to zas and probably an iptable rule to redirect
http traffic from port 80 to the zas port.

Also `zasd` needs to run as a daemon.

Pull requests are very welcome.

## Installation

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

To configure an application with Zas, create a symlink `~/.zas/apps/app_name`
pointing to your application directory.

Make sure the directory has a Procfile that runs a server in a `$PORT`
variable. Also make sure you can run foreman successfully in this directory.

Now you can use `app_name.dev` in your browser to access the application. The
application will start automatically on first access and keep running forever.
The log output is available at `~/.zas/logs/app_name`.

### Terminating Applications

To terminate an application, run:

```
curl -H 'Host: zas.dev' localhost/apps/app_name/term
```

Replacing `app_name` with the name of the symlink.

### CLI

Zas comes with a CLI. Run `zas commands` for a list of possible commands.

## How does it work?

Zas runs a DNS server on port 12043 that resolves `.dev` domains to
`127.0.0.1` and ignores any other domains.

It also has an HTTP server running on port 12044, and the install script sets
up a rule that forwards requests on port 80 to 12044. When Zas receives the
request, it extracts the Host header to determine the name of the app to
proxy the request to.

If the app is not running, Zas chooses a port and calls 'foreman start' on the
app directory with the PORT environment variable set, then it waits for the
port to open.

## Contributing

Clone the repository and make sure the tests run. You'll probably need:

- [Rust](https://www.rust-lang.org) stable
- [bats](https://github.com/sstephenson/bats)
- [foreman](https://github.com/ddollar/foreman)
- [Nodejs](https://nodejs.org)

Pull requests are very welcome.
