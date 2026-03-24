---
name: everything-claude-code-conventions
description: Development conventions and patterns for everything-claude-code. JavaScript project with conventional commits.
---

# Everything Claude Code Conventions

> Generated from [affaan-m/everything-claude-code](https://github.com/affaan-m/everything-claude-code) on 2026-03-24

## Overview

This skill teaches Claude the development patterns and conventions used in everything-claude-code.

## Tech Stack

- **Primary Language**: JavaScript
- **Architecture**: hybrid module organization
- **Test Location**: separate

## When to Use This Skill

Activate this skill when:
- Making changes to this repository
- Adding new features following established patterns
- Writing tests that match project conventions
- Creating commits with proper message format

## Commit Conventions

Follow these commit message conventions based on 500 analyzed commits.

### Commit Style: Conventional Commits

### Prefixes Used

- `fix`
- `feat`
- `docs`
- `test`

### Message Guidelines

- Average message length: ~62 characters
- Keep first line concise and descriptive
- Use imperative mood ("Add feature" not "Added feature")


*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/commands/add-or-update-skill-documentation.md)
```

*Commit message example*

```text
perf(hooks): move post-edit-format and post-edit-typecheck to strict-only (#757)
```

*Commit message example*

```text
fix: safe Codex config sync — merge AGENTS.md + add-only MCP servers (#723)
```

*Commit message example*

```text
docs(zh-CN): translate code block(plain text) (#753)
```

*Commit message example*

```text
security: remove supply chain risks, external promotions, and unauthorized credits
```

*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/commands/feature-development.md)
```

*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/commands/database-migration.md)
```

*Commit message example*

```text
feat: add everything-claude-code ECC bundle (.claude/enterprise/controls.md)
```

## Architecture

### Project Structure: Single Package

This project uses **hybrid** module organization.

### Configuration Files

- `.github/workflows/ci.yml`
- `.github/workflows/maintenance.yml`
- `.github/workflows/monthly-metrics.yml`
- `.github/workflows/release.yml`
- `.github/workflows/reusable-release.yml`
- `.github/workflows/reusable-test.yml`
- `.github/workflows/reusable-validate.yml`
- `.opencode/package.json`
- `.opencode/tsconfig.json`
- `.prettierrc`
- `eslint.config.js`
- `package.json`

### Guidelines

- This project uses a hybrid organization
- Follow existing patterns when adding new code

## Code Style

### Language: JavaScript

### Naming Conventions

| Element | Convention |
|---------|------------|
| Files | camelCase |
| Functions | camelCase |
| Classes | PascalCase |
| Constants | SCREAMING_SNAKE_CASE |

### Import Style: Relative Imports

### Export Style: Mixed Style


*Preferred import style*

```typescript
// Use relative imports
import { Button } from '../components/Button'
import { useAuth } from './hooks/useAuth'
```

## Testing

### Test Framework

No specific test framework detected — use the repository's existing test patterns.

### File Pattern: `*.test.js`

### Test Types

- **Unit tests**: Test individual functions and components in isolation
- **Integration tests**: Test interactions between multiple components/services

### Coverage

This project has coverage reporting configured. Aim for 80%+ coverage.


## Error Handling

### Error Handling Style: Try-Catch Blocks


*Standard error handling pattern*

```typescript
try {
  const result = await riskyOperation()
  return result
} catch (error) {
  console.error('Operation failed:', error)
  throw new Error('User-friendly message')
}
```

## Common Workflows

These workflows were detected from analyzing commit patterns.

### Database Migration

Database schema changes with migration files

**Frequency**: ~2 times per month

**Steps**:
1. Create migration file
2. Update schema definitions
3. Generate/update types

**Files typically involved**:
- `migrations/*`

**Example commit sequence**:
```
Add Turkish (tr) docs and update README (#744)
docs(zh-CN): translate code block(plain text) (#753)
fix(install): add rust, cpp, csharp to legacy language alias map (#747)
```

### Feature Development

Standard feature implementation workflow

**Frequency**: ~21 times per month

**Steps**:
1. Add feature implementation
2. Add tests for feature
3. Update documentation

**Files typically involved**:
- `manifests/*`
- `**/*.test.*`
- `**/api/**`

**Example commit sequence**:
```
docs(pt-BR): add rules translation
docs(pt-BR): add examples translation
docs(pt-BR): add commands translation
```

### Add Or Update Skill

Adds a new skill or updates an existing skill, including documentation and configuration.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update SKILL.md in the appropriate skills/ directory
2. Optionally update related documentation or integration files

**Files typically involved**:
- `skills/*/SKILL.md`
- `docs/zh-CN/skills/*/SKILL.md`
- `docs/tr/skills/*/SKILL.md`

**Example commit sequence**:
```
Create or update SKILL.md in the appropriate skills/ directory
Optionally update related documentation or integration files
```

### Add Or Update Command Doc

Adds or updates documentation for a command, including translations.

**Frequency**: ~3 times per month

**Steps**:
1. Create or update command documentation in the commands/ directory
2. Optionally add or update translations in docs/zh-CN/commands/ or docs/tr/commands/

**Files typically involved**:
- `commands/*.md`
- `docs/zh-CN/commands/*.md`
- `docs/tr/commands/*.md`
- `docs/pt-BR/commands/*.md`

**Example commit sequence**:
```
Create or update command documentation in the commands/ directory
Optionally add or update translations in docs/zh-CN/commands/ or docs/tr/commands/
```

### Add Or Update Localization

Adds or updates documentation translations for a new or existing language.

**Frequency**: ~2 times per month

**Steps**:
1. Add or update translated files in docs/{lang}/ directories for agents, commands, skills, rules, etc.
2. Update README.md to reference the new or updated language

**Files typically involved**:
- `docs/zh-CN/**/*`
- `docs/pt-BR/**/*`
- `docs/tr/**/*`
- `README.md`

**Example commit sequence**:
```
Add or update translated files in docs/{lang}/ directories for agents, commands, skills, rules, etc.
Update README.md to reference the new or updated language
```

### Add Or Update Hook

Adds a new agentic hook or updates hook configuration/scripts.

**Frequency**: ~2 times per month

**Steps**:
1. Edit hooks/hooks.json to add or modify hook definitions
2. Update or add hook implementation scripts in scripts/hooks/
3. Optionally update related plugin files (e.g., .opencode/plugins/ecc-hooks.ts)

**Files typically involved**:
- `hooks/hooks.json`
- `scripts/hooks/*.js`
- `.opencode/plugins/*.ts`

**Example commit sequence**:
```
Edit hooks/hooks.json to add or modify hook definitions
Update or add hook implementation scripts in scripts/hooks/
Optionally update related plugin files (e.g., .opencode/plugins/ecc-hooks.ts)
```

### Add Or Update Session Adapter

Adds or updates session adapter logic, including contract documentation and tests.

**Frequency**: ~2 times per month

**Steps**:
1. Edit scripts/lib/session-adapters/*.js for implementation
2. Update docs/SESSION-ADAPTER-CONTRACT.md for documentation
3. Update or add tests in tests/lib/session-adapters.test.js

**Files typically involved**:
- `scripts/lib/session-adapters/*.js`
- `docs/SESSION-ADAPTER-CONTRACT.md`
- `tests/lib/session-adapters.test.js`

**Example commit sequence**:
```
Edit scripts/lib/session-adapters/*.js for implementation
Update docs/SESSION-ADAPTER-CONTRACT.md for documentation
Update or add tests in tests/lib/session-adapters.test.js
```

### Add Or Update Install Manifest

Extends language support or fixes install resolution logic for legacy or new languages.

**Frequency**: ~2 times per month

**Steps**:
1. Edit scripts/lib/install-manifests.js for logic
2. Update manifests/install-components.json for manifest data
3. Add or update tests in tests/lib/install-manifests.test.js

**Files typically involved**:
- `scripts/lib/install-manifests.js`
- `manifests/install-components.json`
- `tests/lib/install-manifests.test.js`

**Example commit sequence**:
```
Edit scripts/lib/install-manifests.js for logic
Update manifests/install-components.json for manifest data
Add or update tests in tests/lib/install-manifests.test.js
```

### Add Or Update Ecc Bundle

Adds new ECC bundle files for commands, skills, team config, or research playbooks.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update files in .claude/commands/, .claude/skills/, .claude/team/, .claude/research/, .claude/rules/, etc.

**Files typically involved**:
- `.claude/commands/*.md`
- `.claude/skills/*/SKILL.md`
- `.claude/team/*.json`
- `.claude/research/*.md`
- `.claude/rules/*.md`

**Example commit sequence**:
```
Create or update files in .claude/commands/, .claude/skills/, .claude/team/, .claude/research/, .claude/rules/, etc.
```


## Best Practices

Based on analysis of the codebase, follow these practices:

### Do

- Use conventional commit format (feat:, fix:, etc.)
- Follow *.test.js naming pattern
- Use camelCase for file names
- Prefer mixed exports

### Don't

- Don't write vague commit messages
- Don't skip tests for new features
- Don't deviate from established patterns without discussion

---

*This skill was auto-generated by [ECC Tools](https://ecc.tools). Review and customize as needed for your team.*
