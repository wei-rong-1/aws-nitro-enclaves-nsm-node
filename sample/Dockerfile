FROM node:16-alpine

RUN apk update && apk add --no-cache bash gcompat
RUN mkdir -p /app

WORKDIR /app

RUN mkdir -p node_modules

COPY node_modules ./node_modules/
COPY tmp/*.node ./node_modules/aws-nitro-enclaves-nsm-node/
COPY index.js .
COPY package.json .
COPY run.sh .

RUN chmod a+x /app/run.sh

CMD ["/app/run.sh"]