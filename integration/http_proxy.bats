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
