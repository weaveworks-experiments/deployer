#!/bin/sh

set -eu

SRC_PATH=/src

# The following triggers a bug in libgit2, see:
# - https://github.com/rust-lang/cargo/issues/2808
# - https://github.com/libgit2/libgit2/issues/3845
#
# We can probably work around this by not sharing `.cargo` from local
#
# # If we run make directly, any files created on the bind mount
# # will have awkward ownership.  So we switch to a user with the
# # same user and group IDs as source directory.  We have to set a
# # few things up so that sudo works without complaining later on.
# uid=$(stat --format="%u" $SRC_PATH)
# gid=$(stat --format="%g" $SRC_PATH)
# echo "weave:x:$uid:$gid::$SRC_PATH:/bin/sh" >>/etc/passwd
# echo "weave:*:::::::" >>/etc/shadow
# echo "weave	ALL=(ALL)	NOPASSWD: ALL" >>/etc/sudoers

# su weave -c "PATH=$PATH make -C $SRC_PATH BUILD_IN_CONTAINER=false $*"

make -C $SRC_PATH BUILD_IN_CONTAINER=false $*
