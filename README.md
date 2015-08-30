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

```
curl -L https://raw.githubusercontent.com/juanibiapina/zas/master/scripts/install.sh | sh
```
