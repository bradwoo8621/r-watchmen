#!/bin/bash

# define Cargo.toml file path
cargo_toml_path="Cargo.toml"

# check Cargo.toml exists or not
if [ ! -f "$cargo_toml_path" ]; then
    echo "文件 $cargo_toml_path 不存在。"
    exit 1
fi

# get modules folders from members field
members=$(grep 'members = ' "$cargo_toml_path" | sed 's/.*\["\([^]]*\)"\].*/\1/' | tr -d ' ' | tr ',' '\n' | sed 's/"//g')

# iterate over each module and run cloc
while IFS= read -r dir; do
    if [ -d "$dir" ]; then
        echo "Counting: $dir"
        cloc "$dir"
        echo "------------------------"
    else
        echo "Directory $dir not exists."
    fi
done <<< "$members"
