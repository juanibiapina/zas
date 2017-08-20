#!/usr/bin/env bats

load support/test_helper
load support/run_zas
load support/config

@test "process manager: starts without any child processes" {
  children="$(pgrep -P ${pid} || true)"

  [ "$children" = "" ]
}

@test "process manager: displays a message if app is not configured" {
  response="$(curl -s -H 'Host: notexistent.dev' localhost:$ZAS_HTTP_PORT)"

  [ "$response" = "App not configured" ]
}

@test "process manager: terminating apps" {
  curl -s -H 'Host: simple.dev' localhost:$ZAS_HTTP_PORT

  response="$(curl -s -H 'Host: zas.dev' localhost:$ZAS_HTTP_PORT/apps/simple/term)"

  echo $response

  [ "$response" = "OK" ]
  [ "$(pgrep -P "$pid")" = "" ]
}
