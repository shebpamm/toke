#!/usr/bin/env bash

if [ "$1" = "get" ]; then
  toket token read | tr -d '\n'
fi
