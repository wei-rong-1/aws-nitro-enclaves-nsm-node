# `AWS Nitro Enclaves NSM for Node.js`

![https://github.com/wei-rong-1/aws-nitro-enclaves-nsm-node/actions](https://github.com/wei-rong-1/aws-nitro-enclaves-nsm-node/workflows/CI/badge.svg)

> A Node.js addon for interacting with the Nitro Secure Module, which provides Nitro Enclaves with attestation capability. [napi-rs](https://napi.rs/) based, only for Linux.

## Features

* PCR query and manipulation
* Attestation
* Entropy

Please refer to [aws/aws-nitro-enclaves-nsm-api](https://github.com/aws/aws-nitro-enclaves-nsm-api).

## Install this package

```
npm install --save aws-nitro-enclaves-nsm-node

yarn add aws-nitro-enclaves-nsm-node
```

## Support OS

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |


## Methods

**open**, alias `init`, Initialization function.

**close**, alias `exit`, Exit function.

**extendPcr**, NSM `ExtendPCR` operation.

**getPcrDescription**, NSM `DescribePCR` operation.

**lockPcr**, NSM `LockPCR` operation.

**lockPcrs**, NSM `LockPCRs` operation.

**getDescription**, NSM `Describe` operation.

**getAttestationDoc**, NSM `GetAttestationDoc` operation.

**getRandom**, NSM `GetRandom` operation. Returns up to 256 bytes of random data.

## Examples

````
import {
  open,
  close,
  extendPcr,
  getPcrDescription,
  lockPcr,
  lockPcrs,
  getDescription,
  getAttestationDoc,
  getRandom
} from "aws-nitro-enclaves-nsm-node"

const fd = open()

// extendPcr
// getPcrDescription
// lockPcr
// lockPcrs
// getDescription
// getRandom

const attestationDoc = getAttestationDoc(
  fd,
  Buffer.from("hello world"),
  null, // nonce
  Buffer.from("my public key"),
)

console.log(attestationDoc)


close(fd)
````

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@12+` which fully supported `Node-API v4`
- Install `yarn@1.x`

## How to build

- yarn
- yarn build

## Lisence

MIT
