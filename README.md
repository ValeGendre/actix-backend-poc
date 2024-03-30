# Actix backend POC

A simple CRUD exemple for an api endpoint implemented using [Actix](https://actix.rs/) as an HTTP server, [Diesel](https://diesel.rs/) for the ORM and [PostgreSQL](https://www.postgresql.org/).

## 🚨 Warning: Before you start

Latest version of postgresql has a dll which may prevent the Backend from loading...

Please refer to [The real problem](https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857).

A proposed fix is to provide an updated `libint-9.dll` to the target/debug directory. Follow the link in the github issue if you're facing the problem.

## 🚀 Getting started

1. Install rust using [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. Install [postgresql](https://www.postgresql.org/download/) since diesel requires some binaries to operate
3. Install the diesel-cli with `cargo install diesel_cli`
4. Install docker and run the compose
5. Run the diesel migration command `diesel migration run`
6. Build and run using `cargo run`

If you see the following error or any erro code 3, please refer to the warning section above.

    error: process didn't exit successfully: `target\debug\actix-backend.exe` (exit code: 3)

## Making a request

To create a new user, you must provide in the body the following parameters:

```JSON
{
    "firstname": "",
    "lastname": "",
    "emailname": "",
    "administrator": false
}
```

To update an user, the body is the same as for the creation except all fields are optionals.

Now with postman you can the test the following API endpoints :

| METHOD     | URI                   | DESCRIPTION                  |
| ---------- | --------------------- | ---------------------------- |
| **GET**    | /api/v1/users         |   Get all users              |
| **POST**   | /api/v1/users         |   Create a new User          |
| **GET**    | /api/v1/users/:userid |   Get a user from its id     |
| **PATCH**  | /api/v1/users/:userid |   Update an user from its id |
| **DELETE** | /api/v1/users/:userid |   Delete an user from its id |
