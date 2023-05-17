#! /bin/bash

echo "build and pack aws-nitro-enclaves-nsm-node..."
cd ..
yarn build:debug --target=x86_64-unknown-linux-musl
yarn build:debug --target=x86_64-unknown-linux-gnu
yarn pack

mkdir -p sample/tmp/
mv -f *.tgz sample/tmp/aws-nitro-enclaves-nsm-node.tgz
cp -f *.node sample/tmp/

if [ "$1" == "preinstall" ]; then
  exit 0
fi

echo "build sample..."
cd "sample"
yarn install --ignore-scripts && yarn build
cp -f tmp/*.node node_modules/aws-nitro-enclaves-nsm-node/

if [ "$1" != "all" ]; then
  exit 0
fi

echo "build sample docker image..."
docker build -f ./Dockerfile -t aws-nitro-enclaves-nsm-node-sample-server .
