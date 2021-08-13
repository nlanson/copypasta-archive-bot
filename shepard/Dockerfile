FROM node:14.17
WORKDIR /app
COPY package.json ./
RUN npm i typescript -g
RUN npm install
COPY . ./
RUN tsc
CMD ["node", "./dist/index"]