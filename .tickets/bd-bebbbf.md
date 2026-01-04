---
id: bd-bebbbf
title: Make label add idempotent (don't error if label exists)
status: closed
priority: 1
type: bug
created_at: 2026-01-04T14:06:31.881289+00:00
updated_at: 2026-01-04T14:17:22.606662+00:00
closed_at: 2026-01-04T14:17:22.606660+00:00
created_by: kevinchen
created_by_type: human
labels:
- cli
- spec-compliance
---

Per AC-005B.3: WHEN the issue already has the label THE SYSTEM SHALL do nothing (idempotent). Currently cmd_label_add returns an error if the label already exists. Should silently succeed instead.

## Comments

### 2026-01-04T14:17:22.606940+00:00 — kevinchen (human)

Closed: Implemented: label add now silently succeeds if label already exists

