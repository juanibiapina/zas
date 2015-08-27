#!/usr/bin/env bats

load support/test_helper
load support/config

@test "http proxy: starts without any child processes" {
  children="$(pgrep -P ${pid} || true)"

  echo "-----------------"
  echo "$children"
  echo "-----------------"

  [ "$children" = "" ]
}

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
  curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -X GET localhost:$HTTP_PORT/save_headers

  [ "$(cat $HEADERS_FILE)" = "accept: */*
connection: close
host: simple.dev
user-agent: curl/7.43.0
x-customheader: lol" ]
}

@test "http proxy: overwrites connection: keep-alive when forwarding to apps" {
  curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -H 'Connection: keep-alive' -X GET localhost:$HTTP_PORT/save_headers

  [ "$(cat $HEADERS_FILE)" = "accept: */*
connection: close
host: simple.dev
user-agent: curl/7.43.0
x-customheader: lol" ]
}

@test "http proxy: returns connection header unchanged" {
  response="$(curl -i -s -H 'Host: simple.dev' -H 'connection: keep-alive' -X GET localhost:$HTTP_PORT 2>&1 | grep Connection | tr -d "\015")"

  [ "${response}" = "Connection: keep-alive" ]
}
