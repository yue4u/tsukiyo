# NONSTYLE stack

## フロントエンド

- vue3 https://v3.ja.vuejs.org/
- scss https://sass-lang.com/
- typescript https://www.typescriptlang.org/

## バックエンド

- rust https://www.rust-lang.org/
- actix https://actix.rs/
- graphql https://graphql.org/
- DAL http://diesel.rs/
- reverse proxy https://caddyserver.com/
- cache / limit redis https://redis.io/

## データベース

- psql https://www.postgresql.org/

## 開発環境

- docker-compose https://www.docker.com/

## Setup

```sh
docker-compose up
```

### edit hosts file

```sh
sudo vim /etc/hosts
```

add these two lines:

```diff
+++ 127.0.0.1 nonstyle.net
+++ 127.0.0.1 api.nonstyle.net
```

view nonstyle.net in browser

## Ports

| port  | server       |
| ----- | ------------ |
| :4000 | rust api     |
| :3000 | vue frontend |
| :5432 | postgresql   |
| :6379 | redis        |

## Run migrations

```sh
docker-compose run --rm --user $(id -u) backend sh
```

inside container:

```sh
diesel setup
diesel migration run
```
