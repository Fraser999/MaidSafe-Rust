#!/bin/bash

result=0
for i in `find . -name '*.rs'`; do
  rustfmt --skip-children --write-mode=diff $i
  if [ $? -ne 0 ]; then
    result=1
  fi
done

if [ $result -ne 0 ]; then
  echo -e "\e[31mAborting commit due to failed rustfmt check.\e[0m"
fi

exit $result
