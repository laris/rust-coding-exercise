#!/usr/bin/env sh
# find and delete binaries from current working directory, exclude *.sh script files
#find . -perm +111 -type f -exec rm -rf {} \;
find . -perm +111 -type f  -not -iname "*.sh" -exec rm -rf {} \;
