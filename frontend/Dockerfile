# development
FROM node as develop
WORKDIR /app
COPY package*.json ./
RUN yarn install
COPY . .

# build
FROM develop as build
RUN yarn build

# production
FROM nginx:1.17.10-alpine as production
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]