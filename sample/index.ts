import { open, close, getAttestationDoc } from 'aws-nitro-enclaves-nsm-node'

console.log('open nsm...')

const fd = open()

console.log('get attestation document...')

const attestationDoc = getAttestationDoc(
  fd,
  Buffer.from("hello world"),
  null,
  Buffer.from("my public key"),
)

console.log(attestationDoc.toString())

console.log('close nsm...')

close(fd)