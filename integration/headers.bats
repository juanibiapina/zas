#!/usr/bin/env bats

load support/test_helper
load support/config

@test "headers: forwards headers to server with connection: close" {
  response="$(curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -A "curl" -X GET localhost:$ZAS_HTTP_PORT/headers)"

  echo "$response"

  [ "$response" = "accept: */*
connection: close
host: simple.dev
user-agent: curl
x-customheader: lol" ]
}

@test "headers: overwrites connection: keep-alive when forwarding to apps" {
  response="$(curl -s -H 'Host: simple.dev' -H 'X-CustomHeader: lol' -A "curl" -H 'Connection: keep-alive' -X GET localhost:$ZAS_HTTP_PORT/headers)"

  echo "$response"

  [ "$response" = "accept: */*
connection: close
host: simple.dev
user-agent: curl
x-customheader: lol" ]
}

@test "headers: returns connection header unchanged" {
  response="$(curl -i -s -H 'Host: simple.dev' -H 'connection: keep-alive' -X GET localhost:$ZAS_HTTP_PORT 2>&1 | grep Connection | tr -d "\015")"

  [ "$response" = "Connection: keep-alive" ]
}

@test "headers: forwards a 302 correctly" {
  response="$(curl -s -H 'Host: simple.dev' -H 'connection: keep-alive' -o /dev/null -w "%{http_code}" -X GET localhost:$ZAS_HTTP_PORT/302)"

  echo "$response"

  [ "$response" = "302" ]
}

