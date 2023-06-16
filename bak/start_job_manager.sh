#!/bin/bash

EXECUTOR_WORKSPACE=$(cd $(dirname $0); pwd)
echo $EXECUTOR_WORKSPACE
nohup $EXECUTOR_WORKSPACE/rlink-standalone type=JobManager \
  1>$EXECUTOR_WORKSPACE/log/stdout \
  2>$EXECUTOR_WORKSPACE/log/stderr \
  & echo $!>$EXECUTOR_WORKSPACE/pid
