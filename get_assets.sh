#!/bin/bash
set -euo pipefail

rm -r ./assets
wget https://opengameart.org/sites/default/files/roadTiles_water.zip
unzip roadTiles_water.zip -d ./assets
rm ./assets/*.*
mv ./assets/png/* ./assets
rm -r ./assets/png
rm roadTiles_water.zip
