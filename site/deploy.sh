#!/bin/bash
set -xe
deployDir=dist
deployBranch=gh-pages
buildCmd='npm run build -- --no-clean'

rm -rf $deployDir
git fetch origin
git worktree add $deployDir origin/$deployBranch
cleanup() {
  git worktree remove $deployDir -f
}
trap cleanup EXIT

gitref=$(git show-ref HEAD | cut -d' ' -f1)
rm -rf $deployDir/*
$buildCmd

pushd $deployDir
git checkout -- settings.json
git add .
git commit -m "Updating site at $(date)"
git push origin HEAD:$deployBranch
popd
