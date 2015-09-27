#!/usr/bin/env bats

load support/test_helper

@test "errors: when ZAS_DNS_PORT is invalid" {
  export ZAS_DNS_PORT=asdf

  run target/debug/zasd

  [ "$status" -ne 0 ]
  [ "$output" = "Invalid port: asdf" ]
}

@test "errors: when ZAS_HTTP_PORT is invalid" {
  export ZAS_HTTP_PORT=asdf

  run target/debug/zasd

  [ "$status" -ne 0 ]
  [ "$output" = "Invalid port: asdf" ]
}
