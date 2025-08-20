#!/bin/bash

# First variable - determines the branch
touch .env

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
