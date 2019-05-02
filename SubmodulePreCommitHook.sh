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
if [[ -d "scripts" && ! -L "scripts" ]]; then
    printf "${ok}\n"
    for script in scripts/*; do
        if [[ "${script}" != "scripts/travis_wait" ]]; then
            printf "${prefix} Running ${script}...\n"
            ${script}
            exit_code=$?
            if [[ ${exit_code} -ne 0 ]]; then
                printf "${prefix} ${red}Running '${script}' failed.${no_colour}\n"
                exit ${exit_code}
            fi
        fi
    done
    exit 0
else
    printf "${orange}not found${orange}\n"
fi

check() {
    cargo $1 -- --version > /dev/null 2>&1
    if [[ $? -ne 0 ]]; then
        printf "${prefix} ${warning} $1 not available.\n"
        local skip=1
    fi

    if [[ ${skip} ]]; then
        printf "${prefix} ${warning} Skipping $1.\n"
    else
        printf "${prefix} Running cargo "
        printf "%s " $@
        printf "... "
        mkdir -p target
        local output_file=target/.$1.out
        cargo $@ &>${output_file}
        if [[ $? -ne 0 ]]; then
            printf "${failed}\n"
            printf "${prefix} The following issues need to be addressed:\n"
            cat ${output_file}
            exit 1
        else
            printf "${ok}\n"
        fi
    fi
}

check fmt -- --check
check clippy --all-targets
