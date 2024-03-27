#!/bin/bash

OUTPUT=$(/path/to/uptime http://example.com)

if [[ $OUTPUT != *"is up!"* ]]; then
  echo "Alert: $OUTPUT"
fi

