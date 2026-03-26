# Copilot Instructions

This file contains foundational principles, workflows, and core project conventions for the Copilot Bootcamp Capstone project.

## Project Overview

A full-stack application using:
- **Frontend**: SvelteKit with TypeScript, Vitest for testing
- **Backend**: Rust with Cargo

## Development Principles

- Follow Test-Driven Development (TDD) practices
- Maintain code quality with linting and formatting
- Document patterns and decisions for future reference
- Use GitHub Copilot and AI agents for enhanced development

## Agent Usage

- **copilot-customization**: Expert agent for creating GitHub Copilot customizations including agents, instructions, prompts, and MCP integrations
- **Explore**: Fast read-only codebase exploration and Q&A subagent for researching complex questions
- **tdd**: Test-Driven Development specialist for writing tests first, implementing code through Red-Green-Refactor cycles, and fixing failing tests
- **code-review**: Code review and quality improvement specialist for analyzing ESLint/compilation errors, suggesting idiomatic JavaScript/Svelte/Rust patterns, and guiding toward clean maintainable code

## Memory System

- **Persistent Memory**: This file (`.github/copilot-instructions.md`) contains foundational principles and workflows
- **Working Memory**: `.github/memory/` directory contains discoveries and patterns
- During active development, take notes in `.github/memory/scratch/working-notes.md` (not committed)
- At end of session, summarize key findings into `.github/memory/session-notes.md` (committed)
- Document recurring code patterns in `.github/memory/patterns-discovered.md` (committed)
- Reference these files when providing context-aware suggestions

For detailed information about the memory system, see [.github/memory/README.md](memory/README.md).

## Workflows

### Test-Driven Development (TDD)

1. Write failing test first
2. Implement minimal code to pass test
3. Refactor while keeping tests green
4. Document patterns in memory system

### Linting and Fixing

1. Run linters to identify issues
2. Fix errors systematically
3. Document common patterns
4. Update memory system with solutions

### Debugging

1. Reproduce the issue
2. Investigate root cause
3. Document findings in scratch/working-notes.md
4. Implement fix
5. Add pattern to patterns-discovered.md if reusable

## Code Style

- TypeScript: Follow strict type checking
- Rust: Follow Rust conventions and clippy recommendations
- Svelte: Use SvelteKit conventions
- Testing: Write clear, descriptive test names and arrange-act-assert structure

## Resources

- Frontend tests: See `packages/frontend/src/lib/vitest-examples/` for examples
- Memory system: See `.github/memory/README.md` for usage guide
