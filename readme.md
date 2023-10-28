# ToDo App

Just some practice using Rust for web dev.

# Dev

Ensure port 5432 is free for the PostgreSQL container to map to. If running PostgreSQL
```
systemctl stop postgresql
```

Then run the Postgres db with docker compose, see `compose.yml`
```
docker-compose up
```
Optional `-d` flag for daemon  

Optionally connect to the containerised postgres db  
```
docker exec -it to-do-postgres psql -U root to_do
```

You've then got `docker-compose stop` to stop the container and `docker-compose down` to delete the container

Run the app
```
cargo run
```
