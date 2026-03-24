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
feat(ecc2): add crash resume session recovery
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
feat: scaffold ECC 2.0 Rust TUI — agentic IDE control plane
```

*Commit message example*

```text
feat(skills): add santa-method - multi-agent adversarial verification (#760)
```

*Commit message example*

```text
feat: pending instinct TTL pruning and /prune command (#725)
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
feat(rules): add C# language support (#704)
fix: sanitize SessionStart session summaries (#710)
feat: add MCP health-check hook (#711)
```

### Feature Development

Standard feature implementation workflow

**Frequency**: ~16 times per month

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
feat: agent description compression with lazy loading (#696)
feat: add nuxt 4 patterns skill (#702)
feat(rules): add C# language support (#704)
```

### Add Or Update Skill Documentation

Adds a new skill or updates documentation for an existing skill, typically in the form of a SKILL.md file under skills/ or skills/*/SKILL.md, sometimes with translations in docs/xx/skills/*/SKILL.md.

**Frequency**: ~3 times per month

**Steps**:
1. Create or update skills/<skill-name>/SKILL.md
2. Optionally update docs/xx/skills/<skill-name>/SKILL.md for translations
3. Commit with a message referencing the skill and a summary of changes

**Files typically involved**:
- `skills/*/SKILL.md`
- `docs/zh-CN/skills/*/SKILL.md`
- `docs/tr/skills/*/SKILL.md`
- `docs/pt-BR/skills/*/SKILL.md`

**Example commit sequence**:
```
Create or update skills/<skill-name>/SKILL.md
Optionally update docs/xx/skills/<skill-name>/SKILL.md for translations
Commit with a message referencing the skill and a summary of changes
```

### Add Or Update Agent Documentation

Adds or updates documentation for agents, usually in agents/ or docs/xx/agents/ directories.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update agents/<agent-name>.md
2. Optionally update docs/xx/agents/<agent-name>.md for translations
3. Update AGENTS.md if catalog changes
4. Commit changes

**Files typically involved**:
- `agents/*.md`
- `docs/zh-CN/agents/*.md`
- `docs/tr/agents/*.md`
- `docs/pt-BR/agents/*.md`
- `AGENTS.md`

**Example commit sequence**:
```
Create or update agents/<agent-name>.md
Optionally update docs/xx/agents/<agent-name>.md for translations
Update AGENTS.md if catalog changes
Commit changes
```

### Add Or Update Command Documentation

Adds or updates documentation for CLI commands, including translations.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update commands/<command>.md
2. Optionally update docs/xx/commands/<command>.md for translations
3. Commit changes

**Files typically involved**:
- `commands/*.md`
- `docs/zh-CN/commands/*.md`
- `docs/tr/commands/*.md`
- `docs/pt-BR/commands/*.md`

**Example commit sequence**:
```
Create or update commands/<command>.md
Optionally update docs/xx/commands/<command>.md for translations
Commit changes
```

### Add Or Update Language Support

Adds support for a new programming language or updates language-specific rules, including documentation and test files.

**Frequency**: ~2 times per month

**Steps**:
1. Add or update rules/<language>/*.md (coding-style, hooks, patterns, security, testing)
2. Update manifests/install-components.json or scripts/lib/install-manifests.js if needed
3. Add or update tests/lib/install-manifests.test.js
4. Optionally update translations in docs/xx/rules/<language>/*.md
5. Commit changes

**Files typically involved**:
- `rules/*/*.md`
- `manifests/install-components.json`
- `scripts/lib/install-manifests.js`
- `tests/lib/install-manifests.test.js`
- `docs/zh-CN/rules/*/*.md`
- `docs/tr/rules/*/*.md`
- `docs/pt-BR/rules/*/*.md`

**Example commit sequence**:
```
Add or update rules/<language>/*.md (coding-style, hooks, patterns, security, testing)
Update manifests/install-components.json or scripts/lib/install-manifests.js if needed
Add or update tests/lib/install-manifests.test.js
Optionally update translations in docs/xx/rules/<language>/*.md
Commit changes
```

### Add Or Update Localization

Adds or updates documentation translations for a new or existing language (e.g., zh-CN, tr, pt-BR).

**Frequency**: ~2 times per month

**Steps**:
1. Create or update docs/<lang>/* (README.md, AGENTS.md, commands/, agents/, skills/, rules/ etc.)
2. Update README.md to add or update language links and counts
3. Commit changes

**Files typically involved**:
- `docs/zh-CN/**/*`
- `docs/tr/**/*`
- `docs/pt-BR/**/*`
- `README.md`

**Example commit sequence**:
```
Create or update docs/<lang>/* (README.md, AGENTS.md, commands/, agents/, skills/, rules/ etc.)
Update README.md to add or update language links and counts
Commit changes
```

### Add Or Update Hook Script

Adds or updates automation hooks (e.g., for config protection, health checks, etc.), including configuration and test files.

**Frequency**: ~2 times per month

**Steps**:
1. Create or update scripts/hooks/<hook-name>.js
2. Update hooks/hooks.json to register the hook
3. Optionally add or update tests in tests/hooks/ or tests/integration/
4. Commit changes

**Files typically involved**:
- `scripts/hooks/*.js`
- `hooks/hooks.json`
- `tests/hooks/*.test.js`
- `tests/integration/*.test.js`

**Example commit sequence**:
```
Create or update scripts/hooks/<hook-name>.js
Update hooks/hooks.json to register the hook
Optionally add or update tests in tests/hooks/ or tests/integration/
Commit changes
```

### Add Or Update Ecc Bundle

Adds or updates an ECC bundle, which includes a set of configuration, skill, rule, and agent files under .claude/, .agents/, .codex/, etc.

**Frequency**: ~2 times per month

**Steps**:
1. Add or update .claude/skills/<bundle>/SKILL.md
2. Add or update .agents/skills/<bundle>/SKILL.md
3. Add or update .agents/skills/<bundle>/agents/*.yaml
4. Add or update .claude/commands/*.md, .claude/rules/*.md, .claude/team/*.json, .claude/ecc-tools.json, etc.
5. Add or update .codex/agents/*.toml
6. Commit changes

**Files typically involved**:
- `.claude/skills/*/SKILL.md`
- `.agents/skills/*/SKILL.md`
- `.agents/skills/*/agents/*.yaml`
- `.claude/commands/*.md`
- `.claude/rules/*.md`
- `.claude/team/*.json`
- `.claude/ecc-tools.json`
- `.codex/agents/*.toml`

**Example commit sequence**:
```
Add or update .claude/skills/<bundle>/SKILL.md
Add or update .agents/skills/<bundle>/SKILL.md
Add or update .agents/skills/<bundle>/agents/*.yaml
Add or update .claude/commands/*.md, .claude/rules/*.md, .claude/team/*.json, .claude/ecc-tools.json, etc.
Add or update .codex/agents/*.toml
Commit changes
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
