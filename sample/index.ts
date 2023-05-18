import { open, close, getAttestationDoc } from 'aws-nitro-enclaves-nsm-node'

function sleep(s: number) {
  return new Promise((resolve) => {
    setTimeout(resolve, s * 1000);
  });
}

async function main() {
  console.log('open nsm...')
  
  const fd = open()
  
  sleep(20)

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

  sleep(120)
}

main()