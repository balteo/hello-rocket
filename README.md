## Miscellaneous information:

docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
DB_URL=postgres://postgres:mysecretpassword@localhost/postgres
