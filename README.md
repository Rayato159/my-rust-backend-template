# Start Postgres on Docker
```bash
docker pull postgres:16-alpine
```
``` bash
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

# Setting Example
```toml
[server]
port = 80

[database]
host = "localhost"
port = 5432
user = "postgres"
password = "123456"
dbname = "mydb"
schema = "public"
```