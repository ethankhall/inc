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
SUBJECT=`git log --format=%s -n 1 HEAD`
COMMIT_BODY=`git log --format=%b -n 1 HEAD`

BODY=`jq --null-input --argjson commits "$COMMITS" --arg message "${MESSAGE}" --arg ID "${SHA}" '{ "commits": $commits, "message": $message, "commitId": $ID}'`

echo "${BODY}" | curl -s -X POST -H "X-AUTH-TOKEN: ${AUTH_TOKEN}" -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version -d @-

VERSION=`curl -s -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version/$SHA | jq -r '.version'`

jq --null-input --arg MESSAGE "${MESSAGE}" --arg ID "${ID}" --arg EMAIL "${EMAIL}" 

curl -u ethankhall:$GITHUB_API_TOKEN -X "POST" \
    "https://api.github.com/repos/ethankhall/inc/git/tags?name=inc-darwin-0.1.3" \
     -H "Content-Type: application/json; charset=utf-8" \
     -d $'{
  "message": "initial version\\n",
  "object": "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
  "tagger": {
    "name": "Scott Chacon",
    "email": "schacon@gmail.com",
    "date": "2011-06-17T14:53:35-07:00"
  },
  "type": "commit",
  "tag": "v0.0.1"
}'
