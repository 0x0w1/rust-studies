# Version Policy

> Basis: jig default (human-intervention axis), adopted 2026-09-03

## Decision Order
1. Is this a fix inside what the project already does? → `patch`
2. Can people do something new, or did a generation turn over, while everything they already do keeps working? → `minor`
3. Did the value on offer widen, shrink, or change, or must a human step in to keep using it? → `major`

## Grade Definitions
| bump | definition |
|---|---|
| `patch` | A fix inside the existing feature set: bug fixes, wording and documentation changes, internal cleanup |
| `minor` | A capability added, removed, or changed, or a generation replaced. Something new is possible and the old way still works |
| `major` | The value on offer widened, shrank, or changed, or a human must edit config, files, or call sites to keep using it |

## Hard Rules
> A change that raises no error but behaves differently is `major`. Its size does not matter.

> A skill or prompt instruction that changes when the agent speaks is at least `minor`.

## Version Format
- While the major version is `0`, a `major` grade raises the minor position: `v0.Y.Z` → `v0.(Y+1).0`. Before 1.0, both the value on offer and the call sites may change at any time. Grade exactly as after 1.0 and record the `major` grade in the release report.
- After `v1.0.0`, a `major` grade raises the major position. The grace above ends there.
