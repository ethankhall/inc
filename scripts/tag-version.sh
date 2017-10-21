#!/bin/bash -eu

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
AUTHOR_NAME=`git log --format=%an -n 1 HEAD`
AUTHOR_EMAIL=`git log --format=%ae -n 1 HEAD`
COMMIT_TIME=`git log --format=%cI -n 1 HEAD`

BODY=`jq --null-input --argjson commits "$COMMITS" --arg message "${MESSAGE}" --arg ID "${SHA}" '{ "commits": $commits, "message": $message, "commitId": $ID}'`

echo "${BODY}" | curl -s -X POST -H "X-AUTH-TOKEN: ${AUTH_TOKEN}" -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version -d @- > /dev/null

VERSION=`curl -s -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version/$SHA | jq -r '.version'`

TAG_BODY=`jq --null-input --arg SUBJECT "${SUBJECT}" \
  --arg SHA "${SHA}" \
  --arg AUTHOR_NAME "${AUTHOR_NAME}" \
  --arg AUTHOR_EMAIL "${AUTHOR_EMAIL}" \
  --arg COMMIT_TIME "${COMMIT_TIME}" \
  --arg VERSION "${VERSION}" \
  $'{ "message": $SUBJECT, 
      "object": $SHA, 
      "type": "commit", 
      "tag": ("v" + $VERSION),
      "tagger": { 
        "name": $AUTHOR_NAME, 
        "email": $AUTHOR_EMAIL, 
        "date": $COMMIT_TIME
      }
    }'`

curl -u ethankhall:$GITHUB_API_TOKEN -X "POST" \
    "https://api.github.com/repos/ethankhall/inc/git/tags" \
     -H "Content-Type: application/json; charset=utf-8" \
     -d "$TAG_BODY" > /dev/null