#!/bin/bash

# Setup terminal colours and standard messages.
red='\033[0;31m'
orange='\033[0;33m'
green='\033[0;32m'
cyan='\033[0;36m'
no_colour='\033[0m'

prefix="${cyan}[pre_commit]${no_colour}"
error="${red}Error:${no_colour}"
warning="${orange}Warning:${no_colour}"
ok="${green}OK${no_colour}"
failed="${red}Failed${no_colour}"

# Check for instances of "Fix before committing"
printf "${prefix} Checking for instances of 'Fix before committing'... "
todos=$(grep -irn --exclude-dir=target --exclude-dir=.git "fix before committing" .)
if [[ $? -eq 0 ]]; then
    printf "${failed}\n"
    printf "${prefix} The following issues need to be addressed:\n"
    printf "${todos}\n"
    exit 1
else
    printf "${ok}\n"
fi

# Run rustfmt if available.
travis_rustfmt_version=$(sed -n -e '/cargo_install.sh rustfmt/ s/[^0-9]*\([0-9\.]\+\)[^0-9]*/\1/p' .travis.yml)
rustfmt_version=$(rustfmt --version 2>&1)
if [[ $? -ne 0 ]]; then
    printf "${prefix} ${warning} rustfmt not available.\n"
    skip_rustfmt=1
elif [[ ${travis_rustfmt_version} && ! "${rustfmt_version}" =~ "${travis_rustfmt_version}" ]]; then
    printf "${prefix} ${warning} Installed rustfmt version \"${rustfmt_version}\" doesn't match \"${travis_rustfmt_version}\" specified in .travis.yml.\n"
    skip_rustfmt=1
fi

if [[ ${skip_rustfmt} ]]; then
    printf "${prefix} ${warning} Skipping rustfmt.\n"
else
    printf "${prefix} Running 'cargo fmt -- --check'... "
    if [[ $(cargo -- fmt --check 2>&1) ]]; then
        printf "${ok}\n"
    else
        printf "${failed}\n"
        cargo fmt -- --check
        exit 1
    fi
fi

# Run clippy if available.
travis_rust_nightly_version=$(sed -n -e '/- nightly-/ s/ *- *//p' .travis.yml)
travis_clippy_version=$(sed -n -e '/cargo_install.sh clippy/ s/[^0-9]*\([0-9\.]\+\)[^0-9]*/\1/p' .travis.yml)
cargo +${travis_rust_nightly_version} >/dev/null 2>&1
if [[ $? -ne 0 ]]; then
    printf "${prefix} ${warning} rust ${travis_rust_nightly_version} not available.\n"
    skip_clippy=1
else
    clippy_version=$(cargo +${travis_rust_nightly_version} clippy -- --version 2>&1)
    if [[ $? -ne 0 ]]; then
        printf "${prefix} ${warning} clippy not available.\n"
        skip_clippy=1
    elif [[ ${travis_clippy_version} && ! "${clippy_version}" =~ "${travis_clippy_version}" ]]; then
        printf "${prefix} ${warning} Installed clippy version \"${clippy_version}\" doesn't match \"${travis_clippy_version}\" specified in .travis.yml.\n"
        skip_clippy=1
    fi
fi

if [[ ${skip_clippy} ]]; then
    printf "${prefix} ${warning} Skipping clippy.\n"
else
    printf "${prefix} Running 'clippy --all-targets'... "
    mkdir -p target
    clippy_out=target/.clippy.out
    export RUSTFLAGS="-C codegen-units=8"
    cargo +${travis_rust_nightly_version} clippy --target-dir=target/clippy --all-targets &>${clippy_out}
    if [[ $? -ne 0 ]]; then
        printf "${failed}\n"
        printf "${prefix} The following issues need to be addressed:\n"
        cat ${clippy_out}
        exit 1
    else
        printf "${ok}\n"
    fi
fi
