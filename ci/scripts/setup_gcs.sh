#!/bin/bash

set -eux

# Custom image - see fsouza/fake-gcs-server#1164
docker run -d -p 4443:4443 \
    tustvold/fake-gcs-server \
    -scheme http \
    -public-host localhost:4443

echo '{"gcs_base_url": "http://localhost:4443", "disable_oauth": true, "client_email": "", "private_key": ""}' > "/tmp/gcs.json"
