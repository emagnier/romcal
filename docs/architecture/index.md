---
title: Architecture
---

This section contains design documents that describe romcal's internal architecture — data models, composition rules, and implementation decisions — before they are implemented in code.

## Design Documents

### [Liturgical Composition Model](/architecture/liturgical-composition-model)

The main architecture reference (~3 000 lines). Synthesizes the liturgical norms from all reference documents into a unified data model for romcal, organized around three output layers:

- **Layer 1 — Liturgical Calendar**: the liturgical day, as the foundation for Layers 2.
- **Layer 2 Mass — Mass Calendar**: the Mass as celebrated on a civil date, with pre-resolved options and composition rules.
- **Layer 2 Hours — Hours Calendar**: the Hours of the Office as celebrated on a civil date, with overlay and substitution rules.

Covers: choice of Mass (GIRM 355), substitution groups, readings categories (GILM 83), Office composition by rank (GILH 225–240), calendar inheritance (CP), Rust type definitions, and the transformation pipeline.

## Navigation Indexes

These indexes summarize each liturgical reference document section by section, highlighting what is relevant to romcal's implementation. Use them to quickly locate the right §§ when working on calendar rules. They are also particularly useful as context for AI-assisted development.

| Document | Navigation Index | Scope |
| -------- | ---------------- | ----- |
| [GNLY — Liturgical Year and Calendar](/reference/gnly) | [Navigation Index](/reference/gnly-index) | Seasons, ranks, Table of Precedence, transfer rules |
| [CP — Particular Calendars](/reference/cp) | [Navigation Index](/reference/cp-index) | Calendar hierarchy, patron saints, proper celebrations |
| [GIRM — Roman Missal](/reference/girm) | [Navigation Index](/reference/girm-index) | Mass selection, liturgical colors, readings rules |
| [GILM — Lectionary for Mass](/reference/gilm) | [Navigation Index](/reference/gilm-index) | Readings cycles, proper/accommodated/common categories |
| [GILH — Liturgy of the Hours](/reference/gilh) | [Navigation Index](/reference/gilh-index) | Office structure, memorial overlay, psalmody |
| [PS — Easter Feasts](/reference/ps) | [Navigation Index](/reference/ps-index) | Lent, Holy Week, Triduum, Easter Time norms |
