# Contributing to TCC Launcher

Thank you for your interest in contributing to TCC Launcher! This document provides guidelines for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites
- Rust 1.85+ (edition 2024)
- Git
- A GitHub account

### Development Setup
1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/TCC-Launcher.git`
3. Add upstream remote: `git remote add upstream https://github.com/azfarbod50-blip/TCC-Launcher.git`
4. Create a feature branch: `git checkout -b feature/your-feature-name`
5. Make your changes
6. Run tests: `cargo test --workspace`
7. Check formatting: `cargo fmt --all -- --check`
8. Run clippy: `cargo clippy --workspace --all-targets -- -D warnings`
9. Commit your changes
10. Push to your fork and create a Pull Request

## Development Guidelines

### Code Style
- Follow Rust conventions (use `cargo fmt`)
- Run `cargo clippy` before committing
- Write tests for new functionality
- Keep functions small and focused

### Commit Messages
Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for test additions
- `chore:` for maintenance tasks

Example: `feat(auth): add offline account validation`

### Pull Request Process
1. Ensure all CI checks pass
2. Update documentation if needed
3. Add tests for new functionality
4. Request review from maintainers
5. Address review feedback

## Architecture

TCC Launcher is organized as a Cargo workspace with multiple packages:

- `tcc_app` - Main GUI application (Freya)
- `tcc_auth` - Offline authentication
- `tcc_core` - Core launcher logic
- `tcc_db` - Database layer
- `tcc_events` - Event system
- `tcc_java` - Java management
- `tcc_macro` - Procedural macros
- `tcc_mc` - Minecraft utilities
- `tcc_net` - Networking
- `tcc_cluster` - Instance management
- `tcc_content` - Content management
- `tcc_polyplus` - PolyPlus integration
- `polyio` - IO utilities

## Testing

Run tests with:
```bash
cargo test --workspace
```

For specific package:
```bash
cargo test -p tcc_auth
```

## Reporting Issues

When reporting issues, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Logs or error messages

## License

By contributing, you agree that your contributions will be licensed under the GPL-3.0 license.