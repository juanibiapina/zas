#!/usr/bin/env bash

FIXTURES_DIR="${BATS_TEST_DIRNAME}/fixtures"

setup() {
  export ZAS_APP_HOME="${FIXTURES_DIR}/zas_home"
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  pkill -P "$pid" 1>&2 || true
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
}

