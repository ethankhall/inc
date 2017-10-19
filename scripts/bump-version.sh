#!/bin/bash -e

[ -z "$AUTH_TOKEN" ] && echo "Need to set AUTH_TOKEN" && exit 1;

COMMITS=`git rev-list HEAD \
    | head -n 50 \
    | sed 's/^/"/g;s/$/",/' \
    | sed '1s/^/[/;50s/,$/]/' \
    | tr -d '\n'` 

set -u
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

MESSAGE=`git log --format=%B -n 1 HEAD`
SHA=`git rev-list HEAD | head -n 1`

#BODY=`jq --argjson commits "$COMMITS" --arg message $MESSAGE --arg ID $SHA '{ "commits": $commits, "message": $message, "commitId": $ID}'`
BODY=`jq --null-input --argjson commits "$COMMITS" --arg message "${MESSAGE}" --arg ID "${SHA}" '{ "commits": $commits, "message": $message, "commitId": $ID}'`
# curl -s -X POST -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/search/version -d @- | jq -r .version`

echo "${BODY}" | curl -s -X POST -H "X-AUTH-TOKEN: ${AUTH_TOKEN}" -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version -d @-

VERSION=`curl -s -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version/$SHA | jq '.version'`
find $DIR/.. -name Cargo.toml -exec sed -i "s/version = \".*\"/version = \"$VERSION\"/g" {} \;