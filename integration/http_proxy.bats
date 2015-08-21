#!/usr/bin/env bats

load support/test_helper
load support/config

setup() {
  export ZAS_HOME="${FIXTURES_DIR}/zas_home"
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  pkill -P "$pid" 1>&2 || true
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
}

@test "http proxy: starts without any child processes" {
  children="$(pgrep -P ${pid} || true)"

  echo "-----------------"
  echo "$children"
  echo "-----------------"

  [ "$children" = "" ]
}

@test "http proxy: proxies a GET to 'simple' app" {
  response="$(curl -s -H 'Host: simple.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK SIMPLE GET" ]
}

@test "http proxy: proxies a GET to 'other' app" {
  response="$(curl -s -H 'Host: other.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK OTHER GET" ]
}

@test "http proxy: proxies request to two different apps" {
  response="$(curl -s -H 'Host: simple.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK SIMPLE GET" ]

  response="$(curl -s -H 'Host: other.dev' localhost:$HTTP_PORT)"

  [ "$response" = "MOCK OTHER GET" ]
}
