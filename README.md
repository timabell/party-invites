# Party Invites - Rust Web App - STUB

Party Invites is a server-rendered web application built with Rust. It uses Rocket for the web framework, Diesel for the ORM, and Twilio for sending SMS. The application allows users to manage party invitations and automatically sends out RSVP reminders via SMS.

This is a bit of an experiment in writing web apps with the help of ChatGPT4, and a bit of a hobby project.

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

2. Install PostgreSQL (if not installed):

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

3. Install the necessary Rust dependencies:

    ```bash
    cargo build
    ```

4. Run database migrations:

    ```bash
    diesel migration run
    ```

5. Start the application:

    ```bash
    cargo run
    ```

## License

Copyright Tim Abell 2023 all rights reserved. This is not open source software.
