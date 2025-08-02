# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Build and run the server locally
- `cargo test` - Run tests (if any exist)
- `cargo check` - Check code without building
- `cargo clippy` - Run linting

### Docker Development
- `docker-compose up --build` - Build and run with Docker Compose
- Server runs on port 3000 inside container, mapped to `$AFL_SERVER_PORT` on host

### Environment Variables
Required environment variables (see `default_env`):
- `AFL_USER_EMAIL` - User email for AFL model execution
- `AFL_SERVER_PORT` - External port mapping for Docker
- `AFL_SERVICE_NAME` - Docker service name

## Architecture

This is a simple Axum-based web server that serves AFL (Australian Football League) match predictions and team ratings.

### Core Structure
- `src/main.rs` - Entry point, sets up Axum router with two routes
- `src/handlers/` - Request handlers organized by functionality
  - `afl_handler.rs` - Main prediction logic using external AFL library
  - `favicon.rs` - Static favicon serving
- `static/` - Static assets (favicon files)

### Key Dependencies
- **axum** - Web framework for HTTP server
- **afl** - Custom AFL prediction library (external Git dependency)
- **tokio** - Async runtime
- **chrono** - Date/time handling
- **tower-http** - HTTP utilities

### Application Flow
1. Server binds to `0.0.0.0:3000`
2. Root route (`/`) triggers AFL model execution with hardcoded team offsets
3. Model returns predictions, team ratings, and performance metrics
4. Results formatted as HTML table and returned to client
5. Favicon route (`/favicon.ico`) serves static favicon file

### Model Execution
The AFL handler runs a prediction model with:
- Hardcoded team rating offsets for all 18 AFL teams
- Year set to 2025
- User email from environment variable
- Returns formatted predictions and team ratings table

The server logs timestamps for requests and model execution completion.