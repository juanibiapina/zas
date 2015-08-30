#!/usr/bin/env bats

load support/test_helper
load support/config

@test "config: uses ZAS_APP_HOME from an environment variable" {
  response="$(curl -s -H 'Host: zas.dev' localhost:$ZAS_HTTP_PORT)"

  [ "$response" = "ZAS_APP_HOME: ${ZAS_APP_HOME}" ]
}
