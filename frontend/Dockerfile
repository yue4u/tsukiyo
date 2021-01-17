# development
FROM node as develop

# see https://stackoverflow.com/a/52092711
RUN mkdir /cache
WORKDIR /cache

COPY package*.json ./
RUN yarn install

RUN mkdir /app
WORKDIR /app

# build
FROM develop as build
RUN yarn build

# production
FROM nginx:1.17.10-alpine as production
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]