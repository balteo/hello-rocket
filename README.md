## Miscellaneous information

```
docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
export DB_URL=postgres://postgres:mysecretpassword@localhost/postgres
export ROCKET_DATABASES={pg_db={url="postgres://postgres:mysecretpassword@localhost/postgres"}}
```
