#!/usr/bin/env bats

load support/test_helper
load support/config

@test "http proxy: proxies a GET to 'simple' app" {
  response="$(curl -s -H 'Host: simple.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK SIMPLE GET: Url: /" ]
}

@test "http proxy: proxies a GET to 'other' app" {
  response="$(curl -s -H 'Host: other.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK OTHER GET: Url: /" ]
}

@test "http proxy: proxies request to two different apps" {
  response="$(curl -s -H 'Host: simple.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK SIMPLE GET: Url: /" ]

  response="$(curl -s -H 'Host: other.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK OTHER GET: Url: /" ]
}

@test "http proxy: proxies a GET with an URL" {
  response="$(curl -s -H 'Host: simple.dev' localhost:$HTTP_PORT/path)"

  [ "$response" = "MOCK SIMPLE GET: Url: /path" ]
}

@test "http proxy: proxies a POST with an URL" {
  response="$(curl -s -H 'Host: simple.dev' -X POST localhost:$HTTP_PORT/path)"

  [ "$response" = "MOCK SIMPLE POST: Url: /path" ]
}

@test "http proxy: forwards POST data to the server" {
  response="$(curl -s -H 'Host: simple.dev' -X POST localhost:$HTTP_PORT/path -d some_data)"

  [ "$response" = "MOCK SIMPLE POST: Url: /path
some_data" ]
}

@test "http proxy: forwards headers to server with connection: close" {
  response="$(curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -A "curl" -X GET localhost:$HTTP_PORT/headers)"

  echo "$response"

  [ "$response" = "accept: */*
connection: close
host: simple.dev
user-agent: curl
x-customheader: lol" ]
}

@test "http proxy: overwrites connection: keep-alive when forwarding to apps" {
  response="$(curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -A "curl" -H 'Connection: keep-alive' -X GET localhost:$HTTP_PORT/headers)"

  echo "$response"

  [ "$response" = "accept: */*
connection: close
host: simple.dev
user-agent: curl
x-customheader: lol" ]
}

@test "http proxy: returns connection header unchanged" {
  response="$(curl -i -s -H 'Host: simple.dev' -H 'connection: keep-alive' -X GET localhost:$HTTP_PORT 2>&1 | grep Connection | tr -d "\015")"

  [ "$response" = "Connection: keep-alive" ]
}

@test "http proxy: forwards a 302 correctly" {
  response="$(curl -s -H 'Host: simple.dev' -H 'connection: keep-alive' -o /dev/null -w "%{http_code}" -X GET localhost:$HTTP_PORT/302)"

  echo "$response"

  [ "$response" = "302" ]
}

@test "http proxy: displays a message if app is not configured" {
  response="$(curl -s -H 'Host: notexistent.dev' localhost:$HTTP_PORT)"

  [ "$response" = "App not configured" ]
}
