# Changelog

```markdown
## [v1.0.0]
### Added
### Fixed
### Changed
### Removed
### Breaking Changes
```

## [v0.1.10]

### Added
- Introduced `CHANGELOG.md`
- High level `ShamirZero` API
- Zero-copy `shamir_split` and `shamir_combine` implementations

### Changed
- Update benchmarks and unit tests to reflect API updates

### Breaking Changes
- **shamir_split**: optimize shamir_split by accepting pre-allocated output buffers

- Replace the `Vec<Vec<u8>>` return type with a caller-provided `shares_out: &mut [&mut [u8]]` parameter. This eliminates heap allocations during secret splitting, improves performance, and gives callers control over memory layout.

- **shamir_combine**: Now accepts a mutable `secret_out` output buffer for direct manipulation

```diff
- pub fn shamir_split(
-     secret: &[u8],
-     parts: usize,
-     threshold: usize,
- ) -> Result<Vec<Vec<u8>>, ShamirError>
+ pub fn shamir_split(
+     secret: &[u8],
+     parts: usize,
+     threshold: usize,
+     shares_out: &mut [&mut [u8]]
+ ) -> Result<(), ShamirError>
```

```diff
- pub fn shamir_combine(parts: &[Vec<u8>]) -> Result<Vec<u8>, ShamirError>;
+ pub fn shamir_combine(parts: &[&[u8]], secret_out: &mut [u8]) -> Result<(), ShamirError>;
```

## **[v0.1.9]**

### Added
- Benchmarked performance improvements for `shamir_split` and `shamir_combine`

### Changed
- Update license to dual license (MIT and Apache 2.0)
- Zero-copy Lagrange basis computation
- Inline Horner's method
- Remove Vault's Polynomial struct and interpolation logic
- Deprecate logic derived from `shamir.go`

### Removed
- **math.rs**: `Polynomial` struct and `evaluate` functionality
- **math.rs**: `interpolate_polynomial`

## **[v0.1.8]**

### Added
- Introduced `fast-inverse` feature flag, enabled by default

## **[v0.1.2 - v0.1.7]**

### Added
- SBOM generation and SLSA provenance upload artifacts and signatures in `sigstore.json` format for verification
- **crates.io** Trusted publishing and GitHub action workflows for distribution
- Added coverage reporting

### Changed
- **CI / CD** improved integration testing
- Updated development and release process following `git flow` branching strategy

## **[v0.1.1]**

### Added
- GitHub Actions workflows for CI / CD
- Static code analysis and dependency monitoring
- Benchmarks and benchmarking harness using `criterion`
- Fuzz testing
- Unit and integration testing

## **[v0.1.0]**

Initial Release