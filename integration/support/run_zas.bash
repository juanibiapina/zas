#!/usr/bin/env bash

setup() {
  export ZAS_APP_DIR="${FIXTURES_DIR}/apps"
  export ZAS_LOG_DIR="${FIXTURES_DIR}/logs"
  target/debug/zasd 1>&2 &
  pid="$!"
}

teardown() {
  pkill -P "$pid" 1>&2 || true
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
  wait_for_port_close $ZAS_DNS_PORT
  wait_for_port_close $ZAS_HTTP_PORT
  wait_for_port_close 13050
  wait_for_port_close 13051
  wait_for_port_close 13052
}

wait_for_port_close() {
  port="$1"

  while nc -z localhost $port; do
    sleep 0.1
  done
}
