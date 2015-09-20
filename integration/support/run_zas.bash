#!/usr/bin/env bash

setup() {
  export ZAS_APP_DIR="${FIXTURES_DIR}/apps"
  export ZAS_LOG_DIR="${FIXTURES_DIR}/logs"
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  pkill -P "$pid" 1>&2 || true
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
}
