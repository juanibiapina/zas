# Zas

[![Build Status](https://travis-ci.org/juanibiapina/zas.svg?branch=master)](https://travis-ci.org/juanibiapina/zas)

Zas is a tool to help with local web development, inspired by [Pow](http://pow.cx).

It works by running a DNS server that resolves `.dev` domains to your local
machine. Then, when the browser makes requests to `awesome.dev`, zas brings up
your `awesome` application and forwards requests to it.

To configure an application with zas, make sure it has a `Procfile` that runs a
server on a `$PORT` variable and create a symlink to it:

```
cd ~/.zas
ln -s ~/projects/awesome awesome
```

The name of the symlink (awesome) determines the hostname (awesome.dev) that
you use to access the application.

## Installation

**Zas can't run alongsize Pow, make sure you remove one before installing the other**

To install, run:

```
curl -L https://raw.githubusercontent.com/juanibiapina/zas/master/scripts/install.sh | sh
```

To uninstall, run:

```
curl -L https://raw.githubusercontent.com/juanibiapina/zas/master/scripts/uninstall.sh | sh
```

Sudo is required for both operations.

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
