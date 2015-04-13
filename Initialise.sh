#!/bin/bash

echo -e "Initialising submodules\n-----------------------"
git submodule update --init
git submodule foreach 'git checkout master'
echo "==============================================================================="

for i in `grep path .gitmodules | sed 's/.*= //'` ; do
  echo -e "Configuring $i\n------------`echo "$i" | tr [:print:] -`"

  git -C $i remote add dirvine `git -C $i config --get remote.origin.url | sed 's/github.com:Fraser999/github.com:dirvine/'`
  git -C $i remote set-url --push dirvine disable_push
  git -C $i fetch dirvine

  echo ""
  git -C $i remote -v
  echo ""
  git -C $i branch -vva
  echo ""
  git -C $i status
  echo "==============================================================================="
done
