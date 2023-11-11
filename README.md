# Source

https://youtube.com/watch?v=3oN70MzSDfY

## Dev

Before using sqlx :
`export DATABASE_URL=$(pbpaste)`

Create first migration :
`sqlx migrate add "initial database setup"`

Run migration :
`sqlx migrate run`

## Test

`curl http://localhost:8080/quotes`

```sh
curl -svS \
-H 'Content-Type: application/json' \
-X POST \
-d '{"book": "gotttt","quote": "yeééé"}' \
http://localhost:8080/quotes
```
