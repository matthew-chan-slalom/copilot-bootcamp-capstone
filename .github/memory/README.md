# Memory System

## Purpose

This memory system helps track patterns, decisions, and lessons learned during development. It provides a structured way for both developers and AI assistants to maintain context across sessions, making future development more efficient and consistent.

## Two Types of Memory

### Persistent Memory
**Location**: `.github/copilot-instructions.md`

Contains foundational principles, workflows, and core project conventions that remain stable across the entire project lifecycle. This file defines:
- Development principles and values
- Core workflows (TDD, linting, debugging)
- Agent configuration and usage
- Architectural decisions

### Working Memory
**Location**: `.github/memory/`

Contains discoveries, patterns, and learnings accumulated during development. These files evolve as you work:
- **session-notes.md**: Historical summaries of completed sessions (committed)
- **patterns-discovered.md**: Accumulated code patterns and solutions (committed)
- **scratch/working-notes.md**: Active session notes (NOT committed, ephemeral)

## Directory Structure

```
.github/memory/
├── README.md                    # This file - explains the system
├── session-notes.md             # Historical session summaries (committed)
├── patterns-discovered.md       # Accumulated patterns (committed)
└── scratch/
    ├── .gitignore              # Ignores all files in scratch/
    └── working-notes.md        # Active session notes (not committed)
```

## When to Use Each File

### During TDD Workflow

**Active Development** (scratch/working-notes.md):
- Track current test being written
- Note decisions about test structure
- Document edge cases discovered
- Record blockers or questions

**After Test Completion** (patterns-discovered.md):
- Document reusable test patterns
- Note successful mocking strategies
- Record common assertion patterns

**End of Session** (session-notes.md):
- Summarize which features were tested
- Document key testing decisions
- Note test coverage outcomes

### During Linting/Fixing

**Active Work** (scratch/working-notes.md):
- List errors being addressed
- Track linting rules being applied
- Note decisions about rule exceptions

**After Resolution** (patterns-discovered.md):
- Document common lint error solutions
- Record project-specific style decisions
- Note tool configuration patterns

### During Debugging

**Active Investigation** (scratch/working-notes.md):
- Track symptoms and error messages
- Document hypotheses and tests
- Note what did/didn't work

**After Resolution** (patterns-discovered.md):
- Document the root cause
- Record the solution pattern
- Link to related files/components

**End of Session** (session-notes.md):
- Summarize bugs fixed
- Document key insights
- Note any technical debt or follow-ups

## How AI Reads and Applies These Patterns

When you engage with GitHub Copilot or other AI assistants:

1. **Context Loading**: The AI can reference session-notes.md and patterns-discovered.md to understand:
   - What has been tried before
   - Project-specific patterns and conventions
   - Previous decisions and their rationale

2. **Pattern Application**: When solving similar problems, the AI will:
   - Apply documented patterns from patterns-discovered.md
   - Avoid previously unsuccessful approaches
   - Maintain consistency with established conventions

3. **Decision Making**: Using historical context, the AI can:
   - Make choices aligned with previous decisions
   - Suggest improvements based on lessons learned
   - Provide context-aware recommendations

4. **In-Session Help**: During active development, you can:
   - Ask the AI to review your scratch/working-notes.md
   - Request pattern suggestions based on patterns-discovered.md
   - Get help organizing findings for session-notes.md

## The Difference: Committed vs Ephemeral

### session-notes.md (Committed)
- **Purpose**: Historical record of completed sessions
- **Lifecycle**: Created at end of each session
- **Audience**: Future you, team members, AI assistants
- **Content**: Polished summaries, key decisions, outcomes
- **Git**: Committed to version control

### scratch/working-notes.md (Not Committed)
- **Purpose**: Active thinking space during development
- **Lifecycle**: Lives only during active session
- **Audience**: Current you, your current AI assistant
- **Content**: Raw notes, questions, experiments, temporary findings
- **Git**: Excluded by .gitignore - never committed

At the end of each session:
1. Review scratch/working-notes.md
2. Extract key findings and decisions
3. Add a new entry to session-notes.md
4. Update patterns-discovered.md with new patterns
5. Clear or archive scratch/working-notes.md for next session

## Workflow Example

### Starting a Session
1. Open scratch/working-notes.md
2. Fill in "Current Task" and "Approach"
3. Begin development

### During Development
- Add findings to "Key Findings"
- Record decisions in "Decisions Made"
- Note blockers as they arise
- Update "Next Steps" as you progress

### Discovering a Pattern
- If you notice a reusable pattern, add it to patterns-discovered.md immediately
- The pattern stays accessible for future sessions

### Ending a Session
1. Review scratch/working-notes.md
2. Create a new entry in session-notes.md with the summary
3. Move any new patterns to patterns-discovered.md if not already added
4. Clear scratch/working-notes.md (optional, or start fresh next session)

## Best Practices

1. **Be Specific**: Document concrete examples, not vague principles
2. **Link Files**: Reference specific files and line numbers when documenting patterns
3. **Update Promptly**: Add patterns when discovered, not at end of session
4. **Review Regularly**: Periodically review patterns-discovered.md to refine and consolidate
5. **Trust the System**: The scratch/ directory is your safe space for messy thinking
6. **Commit Often**: After updating session-notes.md or patterns-discovered.md, commit them

## Getting Started

1. Copy the template from scratch/working-notes.md
2. Fill in your current task
3. Start taking notes as you work
4. Reference patterns-discovered.md when facing familiar problems
5. At session end, summarize into session-notes.md

The memory system is a tool to enhance your development workflow. Use it as much or as little as helps you work effectively!
