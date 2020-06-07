#!/bin/bash

COLOR_GREEN="\033[32m"
COLOR_RED="\033[31m"
COLOR_YELLOW="\033[33m"
COLOR_DEFAULT="\033[39m"

ERR_MSG="${COLOR_RED}ERROR!${COLOR_DEFAULT}"
COMPLETE_MSG="${COLOR_GREEN}complete!${COLOR_DEFAULT}"

func() {
    local FUNCTION_SOURCE="coal_function.sh"
    local ROOT_DIR="${HOME}/.coal/setup"

    local RC_FILE=$(case $SHELL in
        (*bash)
            echo "${HOME}/.bashrc";;
        (*zsh)
            echo "${HOME}/.zshrc";;
        (*ksh)
            echo "${HOME}/.kshrc";;
        (*)
            # Default to bashrc
            echo "${HOME}/.bashrc";;
    esac)

    printf "Using source file: ${COLOR_YELLOW}${RC_FILE}${COLOR_DEFAULT}\n"

    # Step 1: Install binary to root directory.
    printf "Installing dependencies and binary..."
    cargo install -q --path . --root ${ROOT_DIR}
    if [ $? -ne 0 ]; then
        printf " ${ERR_MSG}\n"
        exit 1
    fi
    printf " ${COMPLETE_MSG}\n"

    # Step 2: Copy over function script and source it.
    printf "Setting up environment..."
    cp $FUNCTION_SOURCE $ROOT_DIR
    if [ $? -ne 0 ]; then
        printf " ${ERR_MSG}\n"
        exit 1
    fi
    source ${ROOT_DIR}/$FUNCTION_SOURCE
    if [ $? -ne 0 ]; then
        printf " ${ERR_MSG}\n"
        exit 1
    fi
    printf " ${COMPLETE_MSG}\n"

    # Step 3: Add sourcing to rc file.
    printf "Setting up source file..."
    echo "source ${ROOT_DIR}/${FUNCTION_SOURCE}" >> ${RC_FILE}
    if [ $? -ne 0 ]; then
        printf " ${ERR_MSG}\n"
        exit 1
    fi
    printf " ${COMPLETE_MSG}\n"
}

# Check for cargo
printf "Checking for cargo..."
if ! command -v "cargo" > /dev/null; then
    printf " ${ERR_MSG}\n"
    printf "Command cargo not detected. Please install cargo and rerun setup.\n"
    exit 1
fi
printf " ${COMPLETE_MSG}\n"

func
unset -f func
printf "Setup complete. Restart your shell for the new changes to take effect.\n"
