#!env bash

function add_member {
    cp Cargo.toml Cargo.toml.bak
    awk -v new_members="$1" '
    BEGIN {
        in_members = 0;
        flag = 0;
        single_line = 0;
    }
    /^members[[:space:]]*=[[:space:]]*\[.*?\][[:space:]]*$/ {
        single_line = flag = 1;
        in_bracket = substr($0, index($0, "[") + 1, index($0, "]") - index($0, "[") - 1);
        split(in_bracket, members, ",");
        printf("members = [\n");
        exists = 0;
        for (i in members) {
            if (members[i] ~ new_members) {
                printf("%s already exists\n", new_members) > "/dev/stderr";
                exit 1;
            }
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", members[i]);
            printf("    %s,\n", members[i]);
        }
        printf("    \"%s\",\n]\n", new_members);
    }
    single_line == 0 {
        missing_comma = 0;
        if (in_members == 1 && /\]/) {
            in_members = 0;
            printf("    \"%s\",\n", new_members);
        } else if (in_members == 1 && $0 ~ new_members) {
            printf("%s already exists\n", new_members) > "/dev/stderr";
            exit 1;
        } else if (in_members == 1 && $0 ~ /"\s*$/) {
            missing_comma = 1;
        }
        printf("%s%c\n", $0, missing_comma ? "," : "");
    }
    /^members[[:space:]]*=[[:space:]]*\[[[:space:]]*$/ { in_members = flag = 1; }
    END {
        if (flag == 0) {
            printf("members = [\n    \"%s\",\n]\n", new_members);
        }
    }' Cargo.toml > Cargo.toml.new && mv Cargo.toml.new Cargo.toml && cargo new $1 --bin
    if [ $? -ne 0 ]; then
        mv Cargo.toml.bak Cargo.toml
        exit 1
    else
        rm Cargo.toml.bak
        cp template/lib.rs $1/src/lib.rs
        if [ -x "$(command -v code)" ]; then
            code $1/src/lib.rs $1/src/main.rs 
        elif [ -x "$(command -v code-insiders)" ]; then
            code-insiders $1/src/lib.rs $1/src/main.rs 
        fi
    fi
}

function rm_member {
    if [ ${#1} -gt 0 ]; then
        if [ ${1:${#1}-1:1} == "/" ]; then
            rm_member=${1:0:${#1}-1}
        else
            rm_member=$1
        fi
    else
        rm_member=$1
    fi
    awk -v rm_member="$rm_member" '
    BEGIN {
        in_members = 0;
        single_line = 0;
        flag = 0
    }
    /^members[[:space:]]*=[[:space:]]*\[.*?\][[:space:]]*$/ {
        single_line = flag = 1;
        in_bracket = substr($0, index($0, "[") + 1, index($0, "]") - index($0, "[") - 1);
        split(in_bracket, members, ",");
        printf("members = [\n");
        for (i in members) {
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", members[i]);
            if (members[i] ~ rm_member) {
                continue;
            }
            if (length(members[i]) > 0) {
                printf("    %s,\n", members[i]);
            }
        }
        printf("]\n");
    }
    single_line == 0 {
        if (in_members == 1 && /\]/) {
            in_members = 0;
            printf("]\n");
        } else if (in_members == 1) {
            if (match($0, rm_member) == 0) {
                printf("%s\n", $0);
            }
        } else {
            printf("%s\n", $0);
        }
    }
    /^members[[:space:]]*=[[:space:]]*\[[[:space:]]*$/ { in_members = flag = 1; }
    END {
        if (flag == 0) {
            printf("members = []\n");
        }
    }' Cargo.toml > Cargo.toml.new || rm Cargo.toml.new && mv Cargo.toml.new Cargo.toml
    if [ $? -ne 0 ]; then
        exit 1
    fi
    if [ -d $1 ]; then
        rm -rf $rm_member
    fi
}

operation=$1

case $operation in
    "add")
        for member in ${@:2}; do
            add_member $member
        done
        ;;
    "rm")
        for member in ${@:2}; do
            rm_member $member
        done
        ;;
    "run")
        for member in ${@:2}; do
            cargo run -p $member
        done
        ;;
    "test")
        for member in ${@:2}; do
            cargo test -p $member
        done
        ;;
    *)
        echo "Usage: $0 [add|rm] <member_name>"
        exit 1
        ;;
esac