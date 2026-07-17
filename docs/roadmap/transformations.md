# Transformations Roadmap

Goal: Comprehensive library of format-independent record transformations.

## Implemented (27)
select, filter, sort, reverse, rename, flatten, fill, explode, dedup, merge, zip, entries, keys, values, round, slice, head, tail, sample, shuffle, pick, search, length, pretty, check, diff, stats, count, unique

## Planned
- group — group records by field value
- aggregate — compute aggregates per group
- pivot — pivot unique values into columns
- unpivot — melt columns into rows
- transpose — swap rows and columns
- normalize — split nested records

## Design
All transformations operate on `serde_json::Value` (Array of Objects).
Behavior must be documented for: nulls, missing fields, mixed types, empty inputs, duplicate keys.
