#!/bin/bash

echo -e "Initialising submodules\n-----------------------"
git submodule update --init
git submodule foreach 'git checkout master'
echo "==============================================================================="

IFS=$(echo -en "\n\b")
mkdir .sublime-projects
for i in `grep path .gitmodules | sed 's/.*= //'` ; do
  echo -e "Configuring $i\n------------`echo "$i" | tr [:print:] -`"

  hook=".git/modules/$i/hooks/pre-commit"
  echo -e '#!/bin/bash\n. "$(git rev-parse --git-path ../../../SubmodulePreCommitHook.sh)"' > "$hook"
  chmod +x "$hook"
  printf '{\n\t"folders":\n\t[\n\t\t{\n\t\t\t"path": "../%s"\n\t\t}\n\t]\n}\n' "$i" > .sublime-projects/$i.sublime-project

  git -C $i remote add upstream `git -C $i config --get remote.origin.url | sed 's/github.com:Fraser999/github.com:maidsafe/'`
  git -C $i remote set-url --push upstream disable_push
  git -C $i fetch upstream

  echo ""
  git -C $i remote -v
  echo ""
  git -C $i branch -vva
  echo ""
  git -C $i status
  echo "==============================================================================="
done
