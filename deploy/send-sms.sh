#!/bin/bash

cd "$(dirname "$0")"
if ! test -e twilio_settings.sh; then
  echo "Missing twilio settings"
  exit 1
fi
source twilio_settings.sh

curl -X POST -d "Body=${2}" \
  -d "From=${TWILIO_NUMBER}" -d "To=${1}" \
  "https://api.twilio.com/2010-04-01/Accounts/${TWILIO_ACCOUNT_SID}/Messages" \
  -u "${TWILIO_API_KEY}:${TWILIO_API_SECRET}"
