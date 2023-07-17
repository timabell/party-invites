# Party Invites - Rust Web App - STUB

Party Invites is a server-rendered web application built with Rust. It uses Rocket for the web framework, Diesel for the ORM, and Twilio for sending SMS. The application allows users to manage party invitations and automatically sends out RSVP reminders via SMS.

This is a bit of an experiment in writing web apps with the help of ChatGPT4, and a bit of a hobby project.

Here's the transcript: <https://chat.openai.com/share/7e3fe058-208c-4300-b975-cdb646156c85> ([cached GPT transcript](gpt-log.txt))

In the end I hit a wall with GPT as it started to forget the context that had been built up and start to produce random and unhelpful outputs. To follow the journey take a look at the [git log](https://github.com/timabell/party-invites/commits/main) (17 July 2023, I don't promise to leave this alone). In total 3 hours elapsed while the tennis was on. In some ways that felt faster, but in other ways it felt like I was wasting time learning the tech properly. I think rather than trying to get GPT to write everything (which appears to be futile currently), it would be better to use GPT to help research some possibilities and suggest some approaches, and then go and read the getting started guides of the relevant crates (such as Rocket and Diesel) and just build manually from there. Then possibly return to GPT to get it to generate specific entities such as `models/invite.rs` and the matching migration sql.

## Features

- Create and manage party events
- Send out party invitations
- Track RSVPs
- Send SMS reminders for RSVPs

## Prerequisites

- Rust 1.70.0 or newer - install with [asdf](https://asdf-vm.com/)
- PostgreSQL

## Installation and Running

1. Clone the repository:

    ```bash
    git clone https://github.com/username/party_invites.git
    cd party_invites
    ```

1. Install PostgreSQL (if not installed):

    1. native:

        ```bash
        # Ubuntu/Debian
        sudo apt-get install postgresql
        # Fedora
        sudo dnf install postgresql
        # macOS (using Homebrew)
        brew install postgresql
        ```

    2. Or Start PostgreSQL in a Docker container:

        ```bash
        docker run --name party_invites_postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
        ```

        This command will download the PostgreSQL image from Docker Hub (if not already downloaded), and start a new container named `party_invites_postgres` running PostgreSQL. It also exposes PostgreSQL's default port (5432) on the host and sets a password for the PostgreSQL `postgres` user. Replace `mysecretpassword` with a secure password.

        Make sure to update your Diesel setup to connect to PostgreSQL with the correct host (localhost if running Docker on the same machine), port (5432), and password.

1. Install the necessary Rust dependencies:

    ```bash
    cargo build
    ```

4. Install Diesel CLI:

    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    asdf reshim
    ```

    This command installs the Diesel command-line interface, which you'll use to run tasks such as database migrations.

1. Run database migrations:

    ```bash
    psql postgres://postgres:mysecretpassword@localhost/ -c 'create database party_invites;'
    DATABASE_URL=postgres://postgres:mysecretpassword@localhost/party_invites diesel migration run
    ```

    If you get the rather confusing "Could not connect to database via ...: Invalid connection url for multiconnection" error it could be anything that means it can't connect to either postgres or the specified database.

1. Start the application:

    ```bash
    cargo run
    ```

## Inspecting the database

```
$ psql postgres://postgres:mysecretpassword@localhost/party_invites
party_invites=# \dt
                    List of relations
┌────────┬────────────────────────────┬───────┬──────────┐
│ Schema │            Name            │ Type  │  Owner   │
├────────┼────────────────────────────┼───────┼──────────┤
│ public │ __diesel_schema_migrations │ table │ postgres │
│ public │ users                      │ table │ postgres │
└────────┴────────────────────────────┴───────┴──────────┘
(2 rows)
```


## License

Copyright Tim Abell 2023 all rights reserved. This is not open source software.
