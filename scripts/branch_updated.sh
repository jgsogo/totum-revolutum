#!/bin/sh

# Check if current branch is up-to-date with master branch

# the output of rev-list in the following test will be empty if there
# are no commits in master that aren't in the current branch

if [ ! -z $(git rev-list ..origin/master) ]
then
    echo "abandoning commit"
    echo "please merge from master and try again"
    exit 1
fi
