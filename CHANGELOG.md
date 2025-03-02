# Changelog

This document records all significant updates and changes to the Kand project.

## [0.2.0] - 2025-03-02

### 🚀 Features

- [**breaking**] Release v0.2.0 with major type system refactoring

### 🐛 Bug Fixes

- *(ci:publish-doc)* Update publish-doc
- *(makefile)* Fix uv-sync, add params for gen_stub.py

### 💼 Other

- Update the types and lib type

## [0.1.3] - 2025-02-27

### 🚜 Refactor

- *(ci:release)* Refactor release ci

## [0.1.2] - 2025-02-27

### 🐛 Bug Fixes

- *(makefile)* Update makefile
- *(bench)* Added #[allow(clippy::expect_used)] to suppress clippy warnings
- *(cdl_gravestone_doji)* Optimize T::from(100).unwrap() to T::from(100).ok_or(KandError::ConversionError)?
- *(var)* Replace unwrap with safe conversion using ok_or(KandError::ConversionError)?

### 🚜 Refactor

- *(ci)* Simplify release workflow and customize changelog footer
- *(tpo)* Replace as f64 with f64::from(u8::try_from(i).unwrap()) for type conversion

### 📚 Documentation

- Update rust doc
- *(helper)* Add missing error documentation for lowest_bars and highest_bars functions

## [0.1.1] - 2025-02-27

### 🚀 Features

- *(ci)* Add changelog ci.

### 🐛 Bug Fixes

- *(aroonosc)* Optimize precision conversion by replacing 'as' with 'T::from' for safety

---

> "Quantitative trading begins with data, thrives on strategy, and succeeds through execution. Kand, making trading simpler."
