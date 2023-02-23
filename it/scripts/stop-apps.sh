#!/bin/bash

wallet_app="dlc-protocol-wallet"
wallet_pid_file="target/$wallet_app.pid"

if [[ -f $wallet_pid_file ]]; then
  pid=$(cat $wallet_pid_file)
  if ps -p $pid > /dev/null; then
    echo "Stopping $wallet_app"
    kill -9 $(cat $wallet_pid_file)
    rm -rf $wallet_pid_file
  fi
fi

oracle_app="sibyls"
oracle_pid_file="target/$oracle_app.pid"

if [[ -f $oracle_pid_file ]]; then
  pid=$(cat $oracle_pid_file)
  if ps -p $pid > /dev/null; then
    echo "Stopping $oracle_app"
    kill -9 $(cat $oracle_pid_file)
    rm -rf $oracle_pid_file
  fi
fi

storage_api_app="storage-api"
storage_api_pid_file="target/$storage_api_app.pid"

if [[ -f $storage_api_pid_file ]]; then
  pid=$(cat $storage_api_pid_file)
  if ps -p $pid > /dev/null; then
    echo "Stopping $storage_api_app"
    kill -9 $(cat $storage_api_pid_file)
    rm -rf $storage_api_pid_file
  fi
fi
