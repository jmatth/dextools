#!/bin/bash -e

WORKTREE_DIR="gh-pages"

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 URL BACKUP_URL"
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

if ! test -d $WORKTREE_DIR; then
  echo "Initializing worktree at $WORKTREE_DIR"
  git fetch
  git worktree add $WORKTREE_DIR gh-pages
fi

echo '=================================='

dexposure-scraper -c ./cache.json -i "$1" -t settings_primary.json -o settings.json &> /dev/null || \
dexposure-scraper -c ./cache.json -i "$2" -t settings_backup.json  -o settings.json &> /dev/null

if /usr/bin/diff -q settings.json $WORKTREE_DIR/settings.json; then
  echo "No changes detected at $(date)."
  exit 0
fi
echo 'Changes detected, updating...'

pushd $WORKTREE_DIR &> /dev/null
git fetch
git reset --hard origin/gh-pages
cp ../settings.json settings.json

git commit -am "Auto update settings at $(date)."
git push
echo "Updated at $(date)."

popd &> /dev/null

./alert.sh
