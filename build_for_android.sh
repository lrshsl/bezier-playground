#!/bin/sh
docker run --rm -v $(pwd)":/root/src" -w /root/src notfl3/cargo-apk cargo quad-apk build --release
