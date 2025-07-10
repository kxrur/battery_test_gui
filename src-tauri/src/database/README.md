# Diesel

[guide for postgresql](https://diesel.rs/guides/getting-started)

## Interact with postgresql

- Add changes to the table / new table to the `src/database/schema.rs` file
- run `diesel migration generate --diff-schema <migration_name>`
- run `diesel migration run`

### If write a `*.sql` file first

This isn't perfect and will likely require a review

- `diesel migration run` to edit postgresql database
- `diesel print-schema > src/database/schema.rs` to generate **schema.rs**
- run `~/.cargo/bin/diesel_ext --model -t > src/database/models.rs` to generate the
  `models.rs` file

## postgresql CLI

- `\dt <table>` shows table info
- `select * from <table>` shows table contents
