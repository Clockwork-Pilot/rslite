# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#feature-build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: clippy_checks](#feature-clippy_checks)
      - [clippy_checks](#clippy_checks)
      - [forbid_clippy_comparison_numeric_lints_any](#forbid_clippy_comparison_numeric_lints_any)
      - [forbid_clippy_stylistic_lints_any](#forbid_clippy_stylistic_lints_any)
      - [forbid_non_camel_case_types_allow](#forbid_non_camel_case_types_allow)
      - [forbid_non_camel_case_types_allow_robust](#forbid_non_camel_case_types_allow_robust)
      - [forbid_non_camel_case_types_any](#forbid_non_camel_case_types_any)
      - [forbid_non_camel_case_types_with_warnings_robust](#forbid_non_camel_case_types_with_warnings_robust)
      - [no_forbidden_allows_robust](#no_forbidden_allows_robust)
    - [Feature: fts5_no_libc_global](#feature-fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
    - [Feature: lib_exports](#feature-lib_exports)
      - [cdylib_symbols](#cdylib_symbols)
      - [no_rust_variadic_exports](#no_rust_variadic_exports)
      - [rslite_expected_exports](#rslite_expected_exports)
    - [Feature: pub_visibility](#feature-pub_visibility)
      - [rslite_uses_crust_core](#rslite_uses_crust_core)
    - [Feature: toolchain_version](#feature-toolchain_version)
      - [stable_toolchain_or_missing](#stable_toolchain_or_missing)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy

### Feature: clippy_checks
**Verify no clippy warnings in rust codebase**

**Goals:**
- Run cargo clippy on all packages
- Ensure all clippy lints pass with no warnings

#### clippy_checks
**Description:** Run cargo clippy and ensure no warnings reported

#### forbid_clippy_comparison_numeric_lints_any
**Description:** Structural: Forbid any allow() attribute (inner #![...] or outer #[...], bare or clippy:: prefix, alone or combined, arbitrary whitespace) for clippy::absurd_extreme_comparisons, almost_swapped, approx_constant, eq_op, excessive_precision, nonminimal_bool across crates/**/*.rs. Also forbid equivalent suppressions in crates/rslite-core/Cargo.toml under [lints.rust] / [lints.clippy] (e.g. `eq_op = "allow"`). Word boundaries prevent false positives on longer identifiers.

#### forbid_clippy_stylistic_lints_any
**Description:** Structural: Forbid any allow() attribute (inner #![...] or outer #[...], bare or clippy:: prefix, alone or combined, arbitrary whitespace) for clippy::empty_line_after_outer_attr and clippy::toplevel_ref_arg across crates/**/*.rs. Also forbid equivalent suppressions in crates/rslite-core/Cargo.toml under [lints.rust] / [lints.clippy] (e.g. `toplevel_ref_arg = "allow"`). Word boundaries prevent false positives on longer identifiers.

#### forbid_non_camel_case_types_allow
**Description:** Structural: Forbid allow() attributes for clippy::non_camel_case_types rule. Types must follow camelCase naming convention.

#### forbid_non_camel_case_types_allow_robust
**Description:** Structural: Robust whitespace-tolerant detection of forbidden clippy::non_camel_case_types allow. Forbids any formatting of allow() with this single lint, ignoring whitespace between allow and (.

#### forbid_non_camel_case_types_any
**Description:** Structural: Forbid any allow() attribute for non_camel_case_types in any form: bare rustc lint or clippy:: prefix, inner (#![...]) or outer (#[...]), alone or combined with other lints, with arbitrary whitespace. Word boundaries prevent false positives on longer identifiers.

#### forbid_non_camel_case_types_with_warnings_robust
**Description:** Structural: Robust whitespace-tolerant detection of forbidden clippy::non_camel_case_types and warnings allows. Forbids any formatting of allow() with non_camel_case_types or warnings, ignoring whitespace between allow and (.

#### no_forbidden_allows_robust
**Description:** Structural: Robust whitespace-tolerant detection of forbidden clippy allows. Detects any formatting of allow() with forbidden lints, ignoring whitespace between allow and (.

### Feature: fts5_no_libc_global
**Ensure fts5.rs contains no libc:: calls (only external C FFI allowed)**

**Goals:**
- fts5.rs must not contain libc:: direct calls
- All C interop must go through safe wrappers or external C FFI only

#### fts5_no_libc_calls
**Description:** fts5.rs must not contain libc:: calls - use safe wrappers or external C FFI instead

### Feature: lib_exports
**All C FFI symbols correctly exported in rlib binaries and cdylib**

**Goals:**
- Verify all 10 exported C FFI symbols are present as global symbols in shell/rustfixture binaries and in the cdylib dynamic symbol table
- Each must have a C source file in c_code/
- Consolidates libs_appendf, libs_snprintf, libs_vsnprintf, libs_vmprintf

#### cdylib_symbols
**Description:** Behavioral: all 10 symbols must appear in dynamic symbol table (T) of the cdylib

#### no_rust_variadic_exports
**Description:** Negative: none of the 10 exported symbols must be Rust variadic extern fns — they live in C only

#### rslite_expected_exports
**Description:** Behavioral: all symbols from expected_exports.txt must appear in cdylib dynamic symbol table

### Feature: pub_visibility
**rslite crate must use crust-core (sqlite_noamalgam) directly, not rslite-raw**

**Goals:**
- Verify rslite crate uses crust-core (sqlite_noamalgam) for FFI
- Ensure no references to rslite-raw in Cargo.toml or source files

#### rslite_uses_crust_core
**Description:** Structural: rslite crate must not reference rslite-raw in its Cargo.toml or source files — it should use crust-core (sqlite_noamalgam) directly

### Feature: toolchain_version
**Enforce Rust toolchain versions. Stable for main code, nightly for shell and tests.**

**Goals:**
- rust-toolchain.toml must use stable channel

#### stable_toolchain_or_missing
**Description:** rust-toolchain.toml must either be missing or use stable channel