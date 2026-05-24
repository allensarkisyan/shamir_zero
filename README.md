# ShamirZero - Pure Rust implementation of Shamir's Secret Sharing Algorithm

<p align="center">
    <picture>
        <img src="https://raw.githubusercontent.com/allensarkisyan/shamir_zero/main/assets/shamir_zero.jpg" alt="ShamirZero">
    </picture>
</p>

Rust implementation of IBM / HashiCorp Vault's Shamir Secret Sharing (originally in Go under MPL-2.0)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE.md)
![Last Commit](https://img.shields.io/github/last-commit/allensarkisyan/shamir_zero)
![CodeQL](https://github.com/allensarkisyan/shamir_zero/workflows/CodeQL/badge.svg?branch=main)
![GitHub issues](https://img.shields.io/github/issues/allensarkisyan/shamir_zero)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/allensarkisyan/shamir_zero/tests.yml?label=tests)
[![codecov](https://codecov.io/gh/allensarkisyan/shamir_zero/graph/badge.svg?token=CMZZBK817L)](https://codecov.io/gh/allensarkisyan/shamir_zero)

## Original Source Code

[`github.com/hashicorp/vault/shamir/shamir.go`](https://github.com/hashicorp/vault/blob/v2.0.1/shamir/shamir.go)

# Getting Started

```rust
use shamir_zero::{shamir_split, shamir_combine};

let secret_key = b"top secret security key";

let secret_shares = shamir_split(secret_key, 5, 2).unwrap();

let recovered = shamir_combine(&secret_shares[0..3]).unwrap();

assert_eq!(secret_key.to_vec(), recovered);
```

# Development & Testing

### Code Quality & Coverage Reporting

```bash
cargo clippy --all-targets &> ./tmp/clippy.log
```

Or use the `clippy-log` alias configured in `.cargo/config.toml`

<br />

```bash
cargo install cargo-tarpaulin
```

```bash
cargo tarpaulin --follow-exec --timeout 60 --branch --out Html --output-dir ./tmp/coverage
```

Or use the `coverage-report` alias configured in `.cargo/config.toml`

```bash
cargo coverage-report
```

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

MIT License

Copyright (c) 2026 Allen Sarkisyan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or would like to contribute to this project,
please open an issue or submit a pull request.

## Author

[Allen Sarkisyan](https://www.linkedin.com/in/allensarkisyan)

Copyright (c) 2026 Allen Sarkisyan. XT-TX. All Rights Reserved.
