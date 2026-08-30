# ACM CSUF OSS Bot

A GitHub bot for the [ACM CSUF Open Source](https://github.com/acmcsufoss)
organization.

The bot provides an HTTP server for receiving GitHub webhooks and interacting
with GitHub through the GitHub API.

## Architecture

```text
GitHub
   │
   │ Webhook
   ▼
┌─────────────────┐
│   Axum Server   │
├─────────────────┤
│     Handlers    │
├─────────────────┤
│    AppState     │
│    └─ Octocrab  │
└─────────────────┘
         │
         ▼
   GitHub REST API
```

The GitHub client is initialized once when the application starts and shared
with handlers through Axum application state.

## Getting started

If you have **[Nix](https://nixos.org/)** (the greatest package manager to
exist), go into the project and run

```bash
nix develop
```

If not, as of writing this make sure you have **rustc** and **cargo** installed.

### Running

Start the server with:

```sh
cargo run
```

The server listens on:

```text
127.0.0.1:8080
```

## API

### `GET /`

Basic application endpoint.

### `GET /health`

Returns the health status of the server.

Example:

```json
{
    "status": "ok",
    "message": "healthy"
}
```

### `GET /prget`

Returns a fixed PR from acmcsuf.com as an example.

```json
do you really think Im going to paste a a billion character output here?
(ok maybe its not a billion characters but its too much for here)
```

### Development

Run tests with:

```sh
cargo test
```

## Project Status

> [!NOTE]
> This project is under active development for the ACM CSUF Open Source
> organization.
>
> The API and internal architecture may change as additional GitHub automation
> is implemented.

## License

This project is licensed under the MIT License. See [`LICENSE`](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Open an issue or submit a pull request with
improvements, bug fixes, or new GitHub automation ideas.
