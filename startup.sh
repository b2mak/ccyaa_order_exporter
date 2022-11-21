#!/bin/bash
set -euo pipefail

./ejson/ejson decrypt ./secrets/secrets.ejson | ./ccyaa_order_exporter "$@"
