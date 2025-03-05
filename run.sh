#!/bin/bash

print_help() {
    echo "$0 <containerizer> <option>"
    echo "containerizers: podman, docker"
    echo "options: build, run"
    echo ""
    echo "example:"
    echo "1: docker run"
    echo "2: docker build"
}

if [ $# -eq 0 ]
     then
        print_help
        exit 1
fi

if [[ $1 != "podman" && $1 != "docker" ]]
     then 
        echo "podman and docker are the only containerizers supported"
        exit 1
fi

CON=$1
TAG=monitor

if [[ $2 == "build" ]]
    then
        echo "building. Result will be gived tag monitor"
        $CON build . -t $TAG 
    elif [ $2 == "run" ]
    then
        echo "Run. You will be placed in an interactive shell"
        $CON run -it --rm $TAG 
    else 
        print_help
fi

