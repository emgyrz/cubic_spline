#!/bin/bash

set -e

cd "$(dirname "${BASH_SOURCE[0]}")" || exit
SCRIPT_DIR=$PWD
PROJECT_DIR=$(dirname "$SCRIPT_DIR")
WEB_DIST_DIR="/tmp/web_dist"
WEB_TPL_DIR="$SCRIPT_DIR/tpl"

cd "$PROJECT_DIR" || exit

rm -rf $WEB_DIST_DIR

wasm-pack build -d $WEB_DIST_DIR

cp -- "$WEB_TPL_DIR"/* "$WEB_DIST_DIR/"

CARGO_PKG_KEYWORDS=$(grep -oP "(?<=keywords =).*" Cargo.toml)

export WEB_DIST_DIR
export CARGO_PKG_KEYWORDS

node "${SCRIPT_DIR}/update_package_json.js"

if [ "$1" = "pub" ]
then
  cd "$WEB_DIST_DIR" || exit
  npm publish
fi

unset WEB_DIST_DIR
unset CARGO_PKG_KEYWORDS
