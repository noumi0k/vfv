# Contributing to vfv

Thank you for your interest in contributing to vfv!

## Development Setup

### Prerequisites

- Rust 1.85 or later
- Git

### Building from Source

```bash
git clone https://github.com/noumi0k/vive-file-viewer.git
cd vive-file-viewer
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Output

```bash
cargo run -- ~/some/directory
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Run tests with `cargo test`
6. Commit your changes with a descriptive message
7. Push to your branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all clippy warnings (`cargo clippy`)
- Keep functions small and focused
- Add comments for non-obvious logic

## Commit Messages

- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 72 characters
- Reference issues when applicable

## Reporting Issues

When reporting issues, please include:

- vfv version (`vfv --version`)
- OS and version
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
