#!/bin/bash

cd "$(dirname "$0")"

num_events=$(< ./settings.json jq '.schedule | length')

while read number; do
  ./send-sms.sh "${number}" "https://dextools.jmatth.com updated with ${num_events} events!"
done < numbers.txt
