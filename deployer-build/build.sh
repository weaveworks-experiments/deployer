#!/bin/sh

set -eu

SRC_PATH=/src

make -C $SRC_PATH BUILD_IN_CONTAINER=false $*
