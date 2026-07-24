#!/bin/bash

majorAdd=0
minorAdd=0
patchAdd=0

case "$1" in
  +001)
    patchAdd=1
    ;;
  +010)
    minorAdd=1
    ;;
  +100)
    majorAdd=1
    ;;
  *)
    echo "arg format error! only support +001/+010/+100 "
    ;;
esac

lastVerTag=`git tag -l --sort=v:refname |grep -E "^v[0-9]+\.[0-9]+\.[0-9]+$"|tail -1`

IFS=. read -r major minor patch <<< "${lastVerTag#v}"

if [ $patchAdd -eq 1 ]; then
   let patch+=1
fi
if [ $minorAdd -eq 1 ]; then
   patch=0
   let minor+=1
fi
if [ $majorAdd -eq 1 ]; then
   patch=0
   minor=0
   let major+=1
fi

echo "lastVerTag: $lastVerTag"
nextVer="${major}.${minor}.${patch}"
nextVerTag="v$nextVer"

echo "nextVerTag: $nextVerTag"
sed -Ei 's/^(version\s*=\s*[^0-9]+)[0-9]+\.[0-9]+.[0-9]+([^0-9]+)$/\1'${nextVer}'\2/' Cargo.toml
git add -A
git commit -m "release $nextVerTag"

git push
git tag $nextVerTag
git push origin $nextVerTag
