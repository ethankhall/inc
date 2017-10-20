#!/bin/bash -ue

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

SHA=`git rev-list HEAD | head -n 1`
# VERSION=`curl --fail -s -H "Content-Type: application/json" http://api.crom.tech/api/v1/project/ethankhall/repo/inc/version/$SHA | jq -r '.version'`
VERSION="0.1.3"

docker build --tag inc-build-image --file $DIR/../Dockerfile --build-arg VERSION=$VERSION --pull $DIR/..

mkdir -p release/$VERSION

docker run inc-build-image /bin/true
LAST_CONTAINER=`docker ps -alq`
echo "Pulling files from ${LAST_CONTAINER}"
docker cp ${LAST_CONTAINER}:/home/builder/output/inc-linux-$VERSION release/$VERSION/inc-linux-$VERSION
docker cp ${LAST_CONTAINER}:/home/builder/output/inc-darwin-$VERSION release/$VERSION/inc-darwin-$VERSION
docker cp ${LAST_CONTAINER}:/home/builder/output/inc-windows-$VERSION.exe release/$VERSION/inc-windows-$VERSION.exe

read -p 'What is the 'Summary' of this release?: ' SUMMARY
read -n 1 -p "Next we'll ask for the body. Please update, save and quit. Press any key to continue."
TEMP_FILE=`mktemp`
echo "There was nothing substansial in this release." > $TEMP_FILE
"${EDITOR:-vi}" $TEMP_FILE
BODY=`cat $TEMP_FILE`
rm $TEMP_FILE

echo "About to create the following release:"
echo "SUMMARY: $SUMMARY"
echo ""
echo "BODY:"
echo $BODY
echo ""
echo "ARTIFACTS:"
for file in release/$VERSION/*
do
  echo "  - $file"
done

echo ""
read -n 1 -p "Does this look correct? [y/N]" CORRECT
echo ""

case $CORRECT in
[nN])
    echo "Not publishing, please re-run!"
    exit 0
    ;;
[yY])
    ;;
*)
    echo "I don't know what you want. Cowardly failing!"
    exit 1
    ;;
esac

CREATE_RELEASE_JSON=`jq --null-input --arg TAG_NAME "v${VERSION}" --arg BODY "${BODY}" --arg SUMMARY "${SUMMARY}" '{"tag_name": $TAG_NAME, "name": $SUMMARY, "body": $BODY, "draft": false, "prerelease": true}'`

UPLOAD_URL=`curl -u ethankhall:$GITHUB_API_TOKEN \
    -X "POST" "https://api.github.com/repos/ethankhall/inc/releases" \
    -H "Content-Type: application/json; charset=utf-8" \
    -d $"$CREATE_RELEASE_JSON" | jq -r .upload_url | cut -f1 -d'{'`


for file in release/$VERSION/*
do
  ARTIFACT_NAME=`basename $file`

  curl -u ethankhall:$GITHUB_API_TOKEN \
    -X "POST" "${UPLOAD_URL}?name=$ARTIFACT_NAME" \
    -H "Content-Type: application/octet-stream" \
    --data-binary @"$file"
done


