#!/bin/bash

echo "Downloading the lastest 'inc' and installing it in ~/bin"
mkdir ~/bin
wget $(curl https://api.github.com/repos/ethankhall/inc/releases/latest | jq -r '.assets[] | select(.name == "inc-mac") | .browser_download_url') -O ~/bin/inc.bck
mv ~/bin/inc.bck ~/bin/inc
chmod +x ~/bin/inc

WHERE_INC=`which inc`
if [ $? -eq 1 ]; then
  echo "Looks like 'inc' didn't end up on your PATH. Please add \$HOME/bin to your PATH."
  exit 1
fi

if [ "$WHERE_INC" != "$HOME/bin/inc" ]; then
  echo "The recent copy of inc isn't first on your path, you may still be using an old version!"
  exit 2
fi
