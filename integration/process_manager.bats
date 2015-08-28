#!/usr/bin/env bats

load support/test_helper
load support/config

@test "process manager: starts without any child processes" {
  children="$(pgrep -P ${pid} || true)"

  [ "$children" = "" ]
}

