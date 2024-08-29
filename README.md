# Rust Backend Template

Author: Ruangyot Nanchiang.

# Introduction

This is a template for a Rust backend server using Axum + SQLx (Postgres).

## Features

- Users
  - [x] Registeration
  - [ ] Login
  - [ ] Logout
  - [ ] Getting
  - [ ] Updating
- Items
  - [ ] Creating
  - [ ] Getting
  - [ ] Listing
  - [ ] Updating
  - [ ] Deleting
- Authentication
  - [ ] JWT

## Start Postgres on Docker

```bash
docker pull postgres:16-alpine
```

```bash
docker run --name mydb -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres:16-alpine
```

```bash
docker exec -it mydb bash
```

```bash
psql -U postgres
```

```bash
CREATE DATABASE mydb;
```

## Setting Example

```toml
[server]
port = 80
timeout = 120
body_limit = 10 # MiB

[database]
host = "localhost"
port = 5432
user = "postgres"
password = "123456"
dbname = "mydb"
schema = "public"
```
