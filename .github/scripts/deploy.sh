#!/usr/bin/env bash

set -e

GITHUB_REPOSITORY="github.com/alexandru/bangs.git"

if [ -z "$GITHUB_TOKEN" ]; then
  echo "ERROR: GITHUB_TOKEN is not set" 1>&2
  exit 1
fi

TEMP_DIR=$(mktemp -d)
echo "Cloning repository into temporary directory: $TEMP_DIR"
rm -rf "$TEMP_DIR"
git clone "https://$GITHUB_TOKEN@$GITHUB_REPOSITORY" "$TEMP_DIR" -b gh-pages

echo "Copying build output to temporary directory"
rsync --filter='P .*' --delete-excluded -Pacv ./build/dist/js/productionExecutable/ "$TEMP_DIR"

echo "Pushing to gh-pages"
cd "$TEMP_DIR" || exit 1
git add .
git config user.name "Alexandru Nedelcu"
git config user.email "noreply@alexn.org"
git commit -m "Update gh-pages"
git push --force --quiet "https://alexandru:$GITHUB_TOKEN@$GITHUB_REPOSITORY" gh-pages:gh-pages
