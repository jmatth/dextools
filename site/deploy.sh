#!/bin/bash
set -xe
deployDir=dist
deployBranch=gh-pages
buildCmd='npm run build'
cname='dextools.jmatth.com'

rm -rf $deployDir
git worktree add $deployDir origin/$deployBranch
cleanup() {
  git worktree remove $deployDir -f
}
trap cleanup EXIT

wtBackup=`mktemp`
cp $deployDir/.git $wtBackup

gitref=$(git show-ref HEAD | cut -d' ' -f1)
rm -rf $deployDir/*
$buildCmd

mv $wtBackup $deployDir/.git
pushd $deployDir
echo $cname > CNAME
git add .
git commit -m "Updating site to ${gitref}"
git push origin HEAD:$deployBranch
popd
