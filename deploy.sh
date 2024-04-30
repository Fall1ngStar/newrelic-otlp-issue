#!/usr/bin/env bash

cargo lambda build --release --arm64 --output-format zip
zip  -u target/lambda/newrelic-otlp-issue/bootstrap.zip collector.yaml
sls deploy
