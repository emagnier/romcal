# Romcal Architecture Review

> Review date: 2026-02-07
> Scope: Core Rust, TypeScript/WASM binding, Python/UniFFI binding, CLI

---

## Executive Summary

Romcal is a well-architected, multi-target Rust project for Catholic liturgical calendar computation. The Rust core (~12,600 lines) compiles to CLI, WebAssembly (TypeScript/JS), and UniFFI (Python), with externalized JSON data. Overall quality is above average for a Rust project, with strong domain modeling and clean separation of concerns. The main structural issues are cross-cutting: JSON string marshalling at FFI boundaries and unstructured error propagation.

---

## 1. Core Rust (`core/`)

### Strengths

- **Domain-Driven Design**: Clean separation `types/` (pure domain model) vs `engine/` (algorithms). Types mirror liturgical concepts precisely (`Precedence` with 27 UNLY levels, `DateDef` untagged enum with 5 date definition modes).
- **Type system**: Rich use of Rust enums, serde attributes (`#[serde(untagged)]`), and feature flags (`bundled-data`, `schema-gen`, `ts-bindings`).
- **Design patterns**: Builder (`Preset` → `Romcal`), Strategy (`EasterCalculationType`), Template Method (season generators in `proper_of_time/`), Locale Fallback with BCP 47 hierarchy.
- **Calendar hierarchy**: Post-order DFS with cycle detection for calendar resolution.
- **Error handling**: `RomcalError` with informative context (available calendars, checked locales).
- **Performance**: `ProperOfTimeCache` for pre-computed dates, `BTreeMap` for sorted output.

### Areas for Improvement

- `engine/calendar.rs` is the largest and most complex file — should be split into `hierarchy.rs`, `merging.rs`, `precedence.rs`.
- Identifiers (`CalendarId`, `LocaleId`) are plain `String` — newtypes would prevent misuse.
- No benchmarks (Criterion) for regression detection.
- No golden-file/snapshot tests for full calendar output.
- Algorithm documentation lacks bibliographic references (UNLY §49, Oudin 1940).

### Score: 8/10

---

## 2. TypeScript/WASM Binding (`bindings/wasm/`, `bindings/typescript/`, `bindings/unplugin/`)

### Strengths

- **3-layer architecture**: WASM bridge → TypeScript wrapper → unplugin optimizer. Each layer has a single responsibility.
- **Type generation**: ts-rs generates 80+ TypeScript types from Rust attributes — single source of truth.
- **Ergonomic API**: 5 `createRomcal()` overloads, async initialization, hides WASM complexity.
- **Tree-shaking**: Subpath exports (`romcal/definitions/france`), per-file data generation.
- **Unplugin**: Virtual module system with build-time hierarchy filtering, HMR cache, 9 bundler adapters.
- **Testing**: Dual environment (Node.js + Playwright browser), plugin tests, ~100+ tests.

### Areas for Improvement

- **JSON string marshalling**: All WASM data returned as `String` → `JSON.parse()`. Should use `serde-wasm-bindgen` for direct `JsValue` serialization.
- **Unstructured errors**: `JsValue::from_str(msg)` — no programmatic error distinction in JS.
- **Naming inconsistency**: `RomcalConfigInterface` (camelCase) vs `RomcalBundle` (snake_case) requires auto-detection.
- **`fix-imports.ts`**: Fragile script looping on `tsc --noEmit` error messages.

### Score: 7.5/10

---

## 3. Python/UniFFI Binding (`bindings/uniffi/`, `bindings/python/`)

### Strengths

- **Proc-macro approach**: UniFFI 0.28 with `#[derive(uniffi::Object)]` — no separate UDL file.
- **Pythonic wrapper**: snake_case, keyword-only args, native enums, type hints, properties.
- **Pydantic v2 models**: Generated from JSON Schema via `datamodel-codegen` — shared schema with TypeScript.
- **Flexible input**: Accepts both Pydantic models and raw dicts.
- **Error chaining**: `raise RomcalError(...) from e` preserves cause (PEP 3134).
- **Distribution**: Maturin wheels for Python 3.11-3.14, Linux/macOS/Windows, x86_64/aarch64.
- **Tests**: ~100+ integration tests covering config, calendar, bundled data, martyrology.

### Areas for Improvement

- **Triple serialization**: `Rust struct` → `serde_json` → `String` (FFI) → `json.loads` → `dict` → `Pydantic model_validate`. Should explore `pythonize` or native UniFFI Records.
- **`MartyrologyQuery`**: Uses `@dataclass` with manual `_to_json_dict()` instead of Pydantic — inconsistent.
- **`ConfigDict(extra="forbid")`**: Breaks forward compatibility if Rust adds fields.

### Score: 7.5/10

---

## 4. CLI (`cli/`)

### Strengths

- **Clap 4 derive**: Type-safe arguments, reusable argument groups (`PresetArgs`, `OutputArgs`), auto-generated help.
- **4-level config precedence**: CLI flags > `--config` file > `.romcal.toml` > XDG config.
- **4 output formats**: YAML (default, custom formatting), JSON, CSV, Lines (colored terminal).
- **Field filtering**: Dot notation (`colors.key`) for nested field extraction.
- **Data merge/replace**: Bundled + custom data with glob patterns, deep merge logic.
- **Schema validation**: Embedded JSON Schema for validating custom definitions/resources.
- **Integration tests**: 22 tests with `assert_cmd` testing the actual binary.

### Areas for Improvement

- **Major duplication**: `calendar.rs` and `masses.rs` share ~95% code (~700 lines). Must extract a generic `render_collection()`.
- **Incomplete `OutputFormat`**: CSV/Lines are `unreachable!()` in `OutputFormat::print()`, implemented ad-hoc in commands.
- **Boolean flags non-negatable**: `--epiphany-on-sunday` cannot be disabled if config TOML says `true`.
- **Silent glob errors**: Failed glob entries only warn on stderr.
- **Sparse documentation**: Few doc-comments on key functions.

### Score: 7/10

---

## 5. Prioritized Action Items

### Priority 1 — High Impact, Active Technical Debt

#### 1.1 Eliminate JSON string marshalling at FFI boundaries
**Cross-cutting**: Core + WASM + UniFFI

| Platform | Current | Target |
|----------|---------|--------|
| WASM | `serde_json::to_string` → `String` → `JSON.parse` | `serde-wasm-bindgen` → `JsValue` directly |
| UniFFI | `serde_json::to_string` → `String` → `json.loads` → `Pydantic` | `pythonize` or native UniFFI Records |
| Core | No shared FFI trait | Add `IntoFfi` trait for centralized conversion |

**Impact**: Performance (365+ entries × 40+ fields per calendar), type safety at FFI boundary.

#### 1.2 Structure errors at FFI boundaries
**Cross-cutting**: Core + WASM + UniFFI + CLI

- **Core**: Define serializable error format: `{ code, message, context }`
- **WASM**: Serialize errors as structured JS objects (not strings)
- **UniFFI**: Map enriched enum variants directly (UniFFI supports data-carrying enums)
- **CLI**: Add distinct exit codes (2 = invalid config, 3 = calendar not found)
- **Python/TS**: Expose error subclasses (`CalendarNotFoundError`, `InvalidYearError`)

#### 1.3 Refactor calendar/masses duplication in CLI
**Scope**: CLI only

Extract generic `render_collection<T: Serialize>()` function. Reduction: ~700 → ~300 lines.

---

### Priority 2 — Quality and Robustness

| # | Task | Scope | Description |
|---|------|-------|-------------|
| 2.1 | Split `engine/calendar.rs` | Core | Extract `hierarchy.rs`, `merging.rs`, `precedence.rs` |
| 2.2 | Complete `OutputFormat` abstraction | CLI | Handle all 4 formats in `OutputFormat::print()` or remove the abstraction |
| 2.3 | Unify `MartyrologyQuery` to Pydantic | Python | Replace `@dataclass` + manual `_to_json_dict()` with `BaseModel` + `model_dump(exclude_none=True)` |
| 2.4 | Change `extra="forbid"` to `extra="ignore"` | Python | Forward compatibility for new fields from Rust |
| 2.5 | Make boolean CLI flags negatable | CLI | Add `--no-epiphany-on-sunday` counterparts |

---

### Priority 3 — Structural Improvements

| # | Task | Scope | Description |
|---|------|-------|-------------|
| 3.1 | Introduce newtypes for identifiers | Core (propagated) | `CalendarId(String)`, `LocaleId(String)` with `#[serde(transparent)]` |
| 3.2 | Add Criterion benchmarks | Core | Calendar generation, Easter computation, precedence resolution, fuzzy search |
| 3.3 | Add golden-file/snapshot tests | Core + CLI | Full calendar output compared to reference fixtures (use `insta` crate) |
| 3.4 | Harden `fix-imports.ts` | TypeScript | Pin TS version, add CI test, or contribute to ts-rs upstream |

---

### Priority 4 — Minor Improvements

| # | Task | Scope | Description |
|---|------|-------|-------------|
| 4.1 | Document algorithms with references | Core | UNLY §49, Oudin 1940, CDWDS documents in doc-comments |
| 4.2 | Document `NO_COLOR` support | CLI | Add `--no-color` flag for discoverability |
| 4.3 | Normalize naming conventions | TypeScript | Explicit `fromBundle()` adapter instead of auto-detection |

> **Note on async**: Adding async wrappers (Python, TypeScript, or Rust) was considered and rejected.
> Romcal is purely CPU-bound with data embedded at compile time — there is no I/O to parallelize.
> Async would only add API complexity, dependency bloat, and runtime overhead with zero benefit.
> This would only become relevant if Romcal loaded data from a remote API or database, which is not planned.

---

## Cross-Cutting Dependency Graph

```
1.1 (JSON marshalling) ──────► 1.2 (structured errors)
         │                              │
         ▼                              ▼
   3.2 (benchmarks)          Python/TS error subclasses
   to measure improvement
         │
         ▼
   3.3 (golden-file tests)
   to prevent regressions

1.3 (calendar/masses dedup) ──► 2.2 (OutputFormat)
                                 both should be done together

3.1 (newtypes) ── independent, can be done anytime
3.4 (fix-imports) ── independent, can be done anytime
```

Items 1.1 and 1.2 should be tackled together as they both modify the FFI boundary layer.
Items 1.3 and 2.2 should be done together as they both affect CLI command rendering.
