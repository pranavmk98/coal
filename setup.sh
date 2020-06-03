#!/bin/bash

func() {
    local ROOT_DIR="${HOME}/.alienv/setup"

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

    # Install binary to root directory.
    cargo install -q --path . --root ${ROOT_DIR}

    # Copy over function script.
    cp alienv_function.sh $ROOT_DIR

    # Source function script and add it to rc file.
    source ${ROOT_DIR}/alienv_function.sh
    echo "source ${ROOT_DIR}/alienv_function.sh" >> ${RC_FILE}
}

if ! command -v "cargo" > /dev/null; then
    echo "Error: cargo command not detected. Please install cargo and rerun setup."
    exit 1
fi

func
unset -f func
echo "Setup complete. Restart your shell for the new changes to take effect."
