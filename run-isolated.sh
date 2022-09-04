#!/bin/bash
podman build  . -t ame-debug

if [ $? -eq 0 ]; then
  podman container exists ame-debug && podman container rm -f ame-debug
  podman run -i -t --name ame-debug ame-debug
fi