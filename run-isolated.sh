#!/bin/bash
podman build  . -t ame-debug \
&& podman container rm ame-debug \
&& podman run -i -t --name ame-debug ame-debug
