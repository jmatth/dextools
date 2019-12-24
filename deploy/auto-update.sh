#!/bin/bash -e

if [ "$#" -ne 3 ]; then
  echo "Usage: $0 URL BACKUP_URL YYYY-mm-dd [Wednesday|Thursday]"
  exit 1
fi

cd "$(dirname "$0")"

panic() {
  if ! test -e ./PANIC && test -n "${ALERT_NUMBER}"; then
    ./send-sms.sh '${ALERT_NUMBER}' 'Dextools deploy broken!'
    touch ./PANIC
  fi
}
trap panic ERR

echo '=================================='

dexposure-scraper -d "$3" -w "$4" -i "$1" -t settings_primary.json -o settings.json &> /dev/null || \
dexposure-scraper -d "$3" -w "$4" -i "$2" -t settings_backup.json  -o settings.json &> /dev/null

if /usr/bin/diff -q settings.json ../site/dist/settings.json; then
  echo "No changes detected at $(date)."
  exit 0
fi
echo 'Changes detected, updating...'

cp settings.json ../site/dist/settings.json

pushd ../site/dist &> /dev/null

git commit -am "Auto update settings at $(date)."
git push -f
echo "Updated at $(date)."

popd &> /dev/null

./alert.sh
