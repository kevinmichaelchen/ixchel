---
id: bd-36e5aa
title: Make label remove warn instead of error when label not present
status: open
priority: 2
type: bug
created_at: 2026-01-04T14:06:35.140085+00:00
updated_at: 2026-01-04T14:06:35.140085+00:00
created_by: kevinchen
created_by_type: human
labels:
- cli
- spec-compliance
---

Per AC-005C.2: WHEN the label is not on the issue THE SYSTEM SHALL display a warning and exit successfully. Currently cmd_label_remove returns an error. Should warn and exit 0 instead.
