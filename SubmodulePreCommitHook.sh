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
    problem_files=()
    modified_files=()

    printf "${prefix} Running 'rustfmt' on committed files... "
    for file in $(git diff --name-only --cached); do
        if [[ ${file: -3} == ".rs" ]]; then
            if [[ $(git diff ${file} 2>&1) ]]; then
                modified_files+=(${file})
            else
                rustfmt --check $file &>/dev/null
                if [[ $? -ne 0 ]]; then
                    problem_files+=(${file})
                fi
            fi
        fi
    done

    if [[ -n "${problem_files}" ]]; then
        printf "${failed}\n"
        printf "${prefix} The following files in this commit need formatting:\n"
        for file in "${problem_files[@]}"; do
            printf "    ${orange}${file}${no_colour}\n"
            rustfmt --check $file
        done
        exit 1
    elif [[ -n "${modified_files}" ]]; then
        printf "${failed}\n"
        printf "${prefix} The following files in this commit have further uncommitted changes:\n"
        for file in "${modified_files[@]}"; do
            printf "    ${orange}${file}${no_colour}\n"
        done
        exit 1
    else
        printf "${ok}\n"
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
    printf "${prefix} Running 'clippy'... "
    mkdir -p target
    clippy_out=target/.clippy.out
    export CARGO_TARGET_DIR=target/clippy
    export RUSTFLAGS="-C codegen-units=8"
    cargo +${travis_rust_nightly_version} clippy &>${clippy_out}
    if [[ $? -ne 0 ]]; then
        printf "${failed}\n"
        printf "${prefix} The following issues need to be addressed:\n"
        cat ${clippy_out}
        exit 1
    else
        printf "${ok}\n"
    fi
    printf "${prefix} Running 'clippy --profile test'... "
    cargo +${travis_rust_nightly_version} clippy --profile test &>${clippy_out}
    if [[ $? -ne 0 ]]; then
        printf "${failed}\n"
        printf "${prefix} The following issues need to be addressed:\n"
        cat ${clippy_out}
        exit 1
    else
        printf "${ok}\n"
    fi
fi
