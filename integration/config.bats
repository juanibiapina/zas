#!/usr/bin/env bats

load support/test_helper
load support/config

@test "config: displays zas home" {
  response="$(curl -s -H 'Host: zas.dev' localhost:$HTTP_PORT)"

  [ "$response" = "ZAS_APP_HOME: ${ZAS_APP_HOME}" ]
}
