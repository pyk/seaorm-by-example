# SeaORM by example

I'm kicking off a blog series about Rust's SeaORM, all from a beginner's angle.

Check out the posts:

1. [Creating PostgreSQL Tables](https://pyk.sh/creating-postgresql-tables-with-rusts-seaorm)
2. [Insert, Select, Update, and Delete Rows in PostgreSQL Tables](https://pyk.sh/rust-seaorm-insert-select-update-and-delete-rows-in-postgresql-tables)

## Getting started

Copy `.env.example` to `.env` and update the values.

Use the following command to run the migration:

```sh
sea-orm-cli migration up
```

Use the following command to update the `entity` crate:

```sh
sea generate entity -o entity/src --lib
```
