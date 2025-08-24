#!/bin/bash

# First variable - determines the branch
touch .env

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"

sed -i '' "/alias rust-llm-configure=/d" ~/.zshrc 2>/dev/null || true
sed -i '' "/alias rust-llm-run=/d" ~/.zshrc 2>/dev/null || true

echo "alias rust-llm-configure=\"cd $SCRIPT_DIR && ./configure.sh\"" >> ~/.zshrc
echo "alias rust-llm=\"f(){ cd $PARENT_DIR && cargo run \"\$@\"; }; f\"" >> ~/.zshrc
echo "Configured aliases in ~/.zshrc"

read -p "Do you want outputs to be local or in obsidian: " CLIENT_TYPE

sed -i '' "/^CLIENT_TYPE=/d" .env && echo "CLIENT_TYPE=$CLIENT_TYPE" >> .env

# Branch based on first variable
if [[ "$CLIENT_TYPE" == "local" ]]; then
  read -p "Where would like to save your outputs?: " OUT_PATH
  sed -i '' "/^OUT_PATH=/d" .env && echo "OUT_PATH=$OUT_PATH" >> .env


elif [[ "$CLIENT_TYPE" == "obsidian" ]]; then
    read -p "Enter your obsidian API key: " OBSIDIAN_API_KEY
    read -p "Enter vault name: " VAULT_NAME
    sed -i '' "/^OBSIDIAN_API_KEY=/d" .env && echo "OBSIDIAN_API_KEY=$OBSIDIAN_API_KEY" >> .env
    sed -i '' "/^VAULT_NAME=/d" .env && echo "VAULT_NAME=$VAULT_NAME" >> .env

fi
