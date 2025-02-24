# About Kand

## The Motivation

TALib has long been a cornerstone for financial indicator calculations, valued for its comprehensive feature set. However, as modern workflows demand higher performance and flexibility, its limitations have become apparent:

- **Performance Bottlenecks**: TALib’s C-based core, while fast, is constrained by Python’s Global Interpreter Lock (GIL), limiting multi-threaded potential in today’s multi-core world. This issue has been a persistent challenge, as noted in discussions around the [Python bindings](https://github.com/TA-Lib/ta-lib-python/issues/675).
- **Complex Setup**: Installing TALib often involves wrangling C library dependencies, a hurdle for users seeking quick deployment. The fact that installation issues dominate their [GitHub issues](https://github.com/TA-Lib/ta-lib-python/issues) speaks volumes about this challenge.
- **Batch-Only Design**: TALib focuses on full-batch computations, lacking efficient incremental updates needed for real-time systems. While its Python bindings offer a stream feature for incremental calculations, it still relies on batch processing underneath, resulting in slower performance. Even attempts to address parallelism in the [native C library](https://github.com/TA-Lib/ta-lib/issues/49) highlight its multi-threading constraints.

These pain points inspired us to rethink how financial tools should work in a modern, high-performance context.

## Why We Built Kand

`kand` was created to address TALib’s shortcomings and deliver a next-generation solution for financial developers. Leveraging Rust’s speed and safety, we set out to build a library that’s not just an alternative, but a leap forward:

- **Elite Performance**: Written in Rust, `kand` matches or exceeds TALib’s speed while adding GIL-free multi-threading for true parallelism.
- **Seamless Integration**: Powered by `rust-numpy`, `kand` shares array memory addresses directly between Python and Rust, enabling true zero-copy data access without any overhead in cross-language operations.
- **Real-Time Ready**: True O(1) complexity with near-zero overhead—each update is just a pure variable computation without loops or batching, making it ideal for real-time streaming data processing.
- **Frictionless Setup**: A single `pip install` command replaces TALib’s cumbersome C setup, with precompiled wheels for all major platforms.
- **Cross-Platform Power**: Runs effortlessly on Linux, macOS, and Windows—musl Linux included.

## Our Vision

`kand` isn’t just about fixing what’s broken—it’s about enabling what’s possible. Whether you’re a quant trader, data scientist, or developer, we aim to provide a tool that’s fast, reliable, and effortless to use, so you can focus on building, not battling your tools.

To see `kand` in action, check out our [Installation Guide](install.md) or dive into the [API Documentation](api.md).
