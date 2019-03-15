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

# Use 'scripts' files if available
printf "${prefix} Checking for 'scripts' folder... "
if [[ -d "scripts" && ! -L "scripts" ]] ; then
    printf "${ok}\n"
    for script in scripts/*; do
        printf "${prefix} Running ${script}...\n"
        ${script}
        if [[ $? -ne 0 ]]; then
            printf "${prefix} ${red}Running '${script}' failed.${no_colour}\n"
            exit $?
        fi
    done
    exit 0
else
    printf "${orange}not found${orange}\n"
fi

# Run rustfmt if available.
rustfmt_version=$(rustfmt --version 2>&1)
if [[ $? -ne 0 ]]; then
    printf "${prefix} ${warning} rustfmt not available.\n"
    skip_rustfmt=1
elif [[ ! "${rustfmt_version}" =~ "-stable" ]]; then
    printf "${prefix} ${warning} Installed rustfmt version \"${rustfmt_version}\" isn't a stable version.\n"
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
clippy_version=$(cargo clippy -- --version 2>&1)
if [[ $? -ne 0 ]]; then
    printf "${prefix} ${warning} clippy not available.\n"
    skip_clippy=1
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
