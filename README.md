Simple web backend written in Rust.

## Table of Contents

-   [Table of Contents](#table-of-contents)
-   [Background](#background)
-   [Install](#install)
-   [Usage](#usage)
-   [TODO](#todo)
-   [Maintainers](#maintainers)
-   [Contributing](#contributing)
    -   [Contributors](#contributors)

## Background

I wanted to be able to port my Typescript web application skills to Rust in a practical way. So, I decided to setup a simple Web_api with Rust that would be a lot faster with solid documentation and error handling. This is my humble attempt at that end.

## Install

This project uses [rust](https://www.rust-lang.org/), [cargo](https://github.com/rust-lang/cargo), and [docker](https://www.docker.com/). Go check them out if you don't have them locally installed.

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

To use this app in its entirety you must have docker running and then run:

```sh
$ docker compose up
# Runs all docker containers
```

To simply run the API run this set of commands:

```sh
$ cd axum_api
# Moves into the api folder
$ cargo run
# Runs cargo to run the api
```

To simply run the Frontend run this set of commands:

```sh
$ TBD
```

## TODO

-   API
    -   Add error handling to API
        -   Build central error handling that all endpoints resolve to
        -   ~~Send User back useful error data
    -   Code split with an effective file structure
    -   ~~Multi-Thread the API with a thread pool
    -   Fill out CRUD operations on all tables
-   Web
    -   Figure out what gonna do for frontend
    -   Do that

## Maintainers

[@AndrewMcDonald-Dev](https://github.com/AndrewMcDonald-Dev).

## Contributing

Feel free to dive in! [Open an issue](https://github.com/AndrewMcDonald-Dev/web_api/issues/new) or submit PRs.

Standard Readme follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.

### Contributors

This project exists thanks to all the people who contribute.
<a href="https://github.com/AndrewMcDonald-Dev/web_api/graphs/contributors">
