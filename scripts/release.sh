#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

if [ "$(uname)" == "Darwin" ]; then
    NAME="release-manager-mac"     
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    NAME="release-manager-linux"
else
    echo "Unable to exec on non Linux or Mac. Bye!"
    exit 1
fi

mkdir -p $DIR/bin
RMCLI=$DIR/bin/release-manager
wget https://github.com/ethankhall/release-manager/releases/download/v0.1.9/$NAME -O $RMCLI
chmod +x $RMCLI

$RMCLI