#!/usr/bin/env bash

wait_for_port_open() {
  port="$1"

  while ! nc -z localhost $port; do
    sleep 0.1
  done
}

wait_for_port_close() {
  port="$1"

  while nc -z localhost $port; do
    sleep 0.1
  done
}

node "integration/fixtures/apps/simple/server.js" 13051 &
wait_for_port_open 13051

node "integration/fixtures/apps/other/server.js" 13052 &
wait_for_port_open 13052

node "integration/fixtures/apps/simple.thing/server.js" 13053 &
wait_for_port_open 13053

export ZAS_DNS_PORT=13043
export ZAS_HTTP_PORT=13044
export XDG_CONFIG_HOME="$(pwd)/integration/fixtures/config"

target/debug/zasd 1>&2 &

bats integration

pkill -P $$ || true

wait_for_port_close $ZAS_DNS_PORT
wait_for_port_close $ZAS_HTTP_PORT
wait_for_port_close 13051
wait_for_port_close 13052
wait_for_port_close 13053
