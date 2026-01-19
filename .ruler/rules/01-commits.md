# Git Commit Hygiene

## 1) Conventional Commit spec

Use the Conventional Commits format:

```
<type>(<scope>): <short, imperative summary>
```

Allowed types (common):

- feat
- fix
- docs
- refactor
- test
- chore
- ci
- build
- perf

Rules:

- Use present-tense, imperative mood ("add", "fix", "refactor").
- Keep the subject concise but meaningful (aim for 50-72 chars).
- Use a scope when the change is localized (crate, module, subsystem).
- If a breaking change exists, add `!` after the type/scope or include a
  `BREAKING CHANGE:` footer in the body.

## 2) High level of detail

Every commit must include a body with concrete, scannable detail:

```
<type>(<scope>): <short summary>

- bullet 1: what changed and why
- bullet 2: key behavior or API change
- bullet 3: notable edge case / follow-up / limitation
```

Guidelines:

- Prefer 3-6 bullets.
- Mention user-facing behavior changes explicitly.
- Include rationale when it is not obvious from the diff.
- Call out migrations, config updates, or data shape changes.

## 3) Exemplary commit

```
feat(helix-embeddings): add provider abstraction with fastembed support

- introduce EmbeddingProvider trait to decouple model implementation
- add fastembed provider with dynamic dimension detection and validation
- expose provider/model metadata for downstream consumers
- document new embedding config fields (provider, dimension)
```
