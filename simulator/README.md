# Offline Simulator

The simulator fixtures let integrators exercise the normalization and policy pipeline without connecting to a live camera network.

`sample_camera_event.json` is intentionally synthetic. It contains an occlusion attribute plus independent restricted-zone and after-hours context so policy tests can demonstrate that occlusion alone is not treated as criminal conduct.

## Adapter test

```bash
cargo test -p flock-adapter
```

## Signal test

The Rust unit tests in `signal-features` generate deterministic sine-wave sensor observations in memory; no biometric or real-person dataset is required.

## Safety rule

Simulator data must never be represented as an actual Flock event or actual person observation. It exists only for development, demonstrations, and reproducible testing.
