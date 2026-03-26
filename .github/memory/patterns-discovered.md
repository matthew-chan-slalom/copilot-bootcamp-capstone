# Patterns Discovered

This file documents reusable code patterns, solutions, and conventions discovered during development. Each pattern includes context, problem statement, solution, and examples. These patterns help maintain consistency and guide future implementation decisions.

**Note**: This file is committed to git and accumulates learnings over time.

---

## Pattern Template

### Pattern: [Pattern Name]

**Context:**
- [When/where this pattern applies]
- [Technologies or frameworks involved]

**Problem:**
- [What problem does this pattern solve?]
- [What challenges does it address?]

**Solution:**
- [The pattern or approach]
- [Key implementation details]

**Example:**
```[language]
// Example code demonstrating the pattern
```

**Related Files:**
- [List of files where this pattern is used]

**Notes:**
- [Additional considerations, gotchas, or variations]

---

## Discovered Patterns

### Pattern: Service Initialization - Empty Array vs Null

**Context:**
- Initializing service state in TypeScript/JavaScript applications
- Managing collections that may start empty vs undefined
- Frontend state management (Svelte stores, React hooks, etc.)

**Problem:**
- Need to distinguish between "no data loaded yet" (null/undefined) and "loaded but empty" ([])
- Avoid null pointer errors when iterating over uninitialized collections
- Provide clear semantics for different states in the UI

**Solution:**
Use `null` for uninitialized/not-yet-loaded state, and `[]` (empty array) for successfully loaded but empty collections:

```typescript
// Service/Store initialization
let items: Item[] | null = null;  // Not loaded yet

async function loadItems() {
  items = null;  // Show loading state
  const result = await fetchItems();
  items = result;  // Could be [] if no items found
}

// UI rendering
{#if items === null}
  <Loading />
{:else if items.length === 0}
  <EmptyState message="No items found" />
{:else}
  <ItemList {items} />
{/if}
```

**Benefits:**
- Clear distinction between loading, empty, and populated states
- Type safety with explicit null handling
- Better user experience (can show loading vs empty states)
- Prevents accidental iteration over undefined

**Related Files:**
- N/A (pattern not yet applied in codebase)

**Notes:**
- Consider using a more explicit state object for complex scenarios:
  ```typescript
  type DataState<T> = 
    | { status: 'loading' }
    | { status: 'loaded', data: T[] }
    | { status: 'error', error: Error };
  ```
- Some teams prefer `undefined` over `null` for consistency with TypeScript conventions
- Be consistent across the codebase - document the chosen convention in copilot-instructions.md

---

_Add new patterns below this line, newest first_

