#!/bin/bash
set -euo pipefail

function load_opengameart_pack {
    wget https://opengameart.org/sites/default/files/$1
    unzip $1 -d ./dump
    mv ./dump/png/* ./assets
    rm -rf ./dump $1
}

rm -rf ./assets
mkdir assets
load_opengameart_pack roadTiles_water.zip
load_opengameart_pack roadTiles_nova.zip
