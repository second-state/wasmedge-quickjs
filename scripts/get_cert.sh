#!/bin/bash

# Check if a domain is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <domain>"
    exit 1
fi

# Retrieve and print the combined TLS certificates
openssl s_client -showcerts -connect "$1":443 2>/dev/null < /dev/null | awk '/BEGIN CERTIFICATE/,/END CERTIFICATE/{print}'