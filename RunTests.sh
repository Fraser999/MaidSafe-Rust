#!/bin/bash
function run_cargo {
  local FileName="$OutputDir/$(echo -e "${@}" | tr -d '[[:space:]]')"
  echo "  Running cargo $@ on the $Type channel..."
  multirust run $Type cargo "$@" > "$FileName.log" 2>"$FileName.err"
}

function run_all {
  for Type in stable beta nightly; do
    local OutputDir="$Dir/$Submodule/$Type"_channel
    mkdir -p "$OutputDir"
    multirust run $Type cargo clean
    run_cargo build
    run_cargo test
    multirust run $Type cargo clean
    run_cargo build --release
    run_cargo test --release
  done
}

Dir=/tmp/BuildResults-$(date +%F-%X)
RootDir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
echo $RootDir
IFS=$(echo -en "\n\b")
for Path in $RootDir/*; do
  [ -d "${Path}" ] || continue # if not a directory, skip
  Submodule="$(basename "${Path}")"
  echo "Entering $Submodule"
  cd $RootDir/$Submodule
  run_all
done

echo "Finished.  Results can be found in $Dir"
