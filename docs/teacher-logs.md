# Teacher Session Logs

This file contains logs of all teaching sessions from the `/teach` agent. Each log includes a timestamp, summary, and the full teaching content.

---

## [2026-03-29 15:45] Component Extraction - Understanding presentation vs. logic separation in Svelte components

# Teaching Session: Component Extraction - Recipes.svelte

## Lens 1: The What

We just refactored the Body component by extracting the "Recipes" section into its own dedicated component file (`Recipes.svelte`). Previously, `Body.svelte` contained both the Favorites section and the Recipes grid with all their markup and logic inline. Now `Body.svelte` acts as a container that orchestrates two child components: `Favorites` and `Recipes`. The recipes grid still has drag-and-drop functionality, but the implementation details now live in a separate, more focused component.

---

## Lens 2: The Why

| Decision Made | Why This Approach | What Was Rejected & Why |
|---|---|---|
| Extract to separate component | Single Responsibility Principle - each component has one job. Makes the codebase easier to navigate and test. | Keeping everything in Body.svelte - works but violates SRP; harder to maintain as features grow |
| Pass event handlers as props (`handleConsider`, `handleFinalize`) | Parent component (`Body`) owns the state and logic, child just presents it. Keeps data flow explicit and predictable. | Moving state into Recipes - creates duplication with Favorites; harder to coordinate between sections |
| Use `$props()` rune for typed props | Svelte 5 runes provide better type safety and clearer component API than the old `export let` syntax | `export let` - old Svelte 4 pattern, less type-safe, being phased out |
| Define Recipe interface in child component | The child knows what shape it needs; parent doesn't need to know internal type details | Shared type file - premature abstraction; only one component uses this interface currently |

The core principle here: **presentation vs. logic separation**. Body.svelte orchestrates (logic), child components present (UI).

---

## Lens 3: Edge Cases & Assumptions

```
ASSUMPTION: The Recipe interface matches the JSON data structure
RISK: If recipes.json changes shape (e.g., adds new required fields), this component won't know until runtime

ASSUMPTION: Event handlers will always be provided by the parent
RISK: If you try to use Recipes.svelte elsewhere without handlers, TypeScript will catch it, but you'll get a runtime error if handlers are undefined

EDGE CASE: What if items array is empty?
→ Current behavior: Shows an empty grid (no fallback message)
→ Is this intentional? Unlike Favorites which has an {:else} block for empty state, Recipes doesn't guide the user when there are no recipes

EDGE CASE: What if a recipe doesn't have an image?
→ Current behavior: Will show broken image or empty space
→ No fallback placeholder image defined

ASSUMPTION: Recipe IDs are unique and stable
RISK: The {#each} keying uses recipe.id - if IDs duplicate or change, drag-and-drop will break
```

---

## Lens 4: Codebase Connection

```
WHAT THIS AFFECTS:
  → Body.svelte (modified - now imports and renders Recipes component)
  → Recipes.svelte (created - new component file)
  → recipes.json (depends on - data source for recipe items)
  → svelte-dnd-action (depends on - drag-and-drop library)
  → Card components (depends on - UI primitives from shadcn-svelte)

WHAT COULD BREAK IF THIS IS WRONG:
  → Drag-and-drop between Recipes and Favorites - both sections coordinate through the parent's state
  → Any future feature that needs to filter/sort recipes - logic lives in Body, not Recipes
  → Type safety - if the Recipe interface diverges from actual JSON structure, runtime errors

WHAT THIS DOES NOT AFFECT (but looks like it might):
  → Favorites.svelte - completely independent component with its own types and handlers
  → The recipes.json file location - still loaded in Body.svelte at the parent level
  → Routing or page-level layout - this is just a presentation component
```

---

## Lens 5: Concepts to Own

```
CONCEPT: Component Composition
  What it is: Breaking UI into smaller, reusable pieces that combine to form larger interfaces
  Why it matters here: Body.svelte went from a 50-line monolith to a 20-line orchestrator. 
  Each child component is now easier to reason about, test, and reuse.
  Further reading: React "Thinking in Components" applies to Svelte too

CONCEPT: Props vs. Bindable State
  What it is: Props flow down (parent → child), bindables flow both ways ($bindable in Svelte 5)
  Why it matters here: Recipes uses one-way props because it doesn't need to modify state - 
  it just calls parent handlers. Favorites uses bind:favorites because it mutates state directly.
  This is a conscious choice about data ownership.

CONCEPT: TypeScript Interfaces for Component APIs
  What it is: Defining explicit types for what a component expects (its "contract")
  Why it matters here: The interface Recipe + interface Props makes it impossible to 
  pass wrong data to Recipes.svelte. The type error we fixed (id: number vs string) 
  was caught by TypeScript before we even ran the code.

CONCEPT: Svelte 5 Runes ($props, $state, $bindable)
  What it is: New reactivity primitives replacing export let, $:, and component bindings
  Why it matters here: `let { items, handleConsider, handleFinalize }: Props = $props()` 
  is the Svelte 5 way. It's more explicit, type-safe, and compiles to faster code than 
  the old export let pattern.
```

---

## Lens 6: Check-In Questions

Before you move on, make sure you can answer these:

1. **If you wanted to add a "clear all recipes" button, where would that logic live - in Body.svelte or Recipes.svelte? Why?**  
   (Hint: Think about who owns the `items` state)

2. **Why did we pass `handleConsider` and `handleFinalize` as props instead of defining them inside Recipes.svelte?**  
   (Hint: What would happen to the drag-and-drop state coordination between Recipes and Favorites?)

3. **What would break if we changed `id: number` to `id: string` in the Recipe interface?**  
   (Hint: Look at recipes.json and how the data flows from parent to child)

---

## Code Quality Notes

🟡 **WORTH ADDRESSING**: Missing empty state in Recipes  
   Where: Recipes.svelte, the {#each} block  
   Trade-off: Favorites has an {:else} placeholder for when it's empty. Recipes doesn't.  
   Consider: Add `{:else}<p>No recipes available</p>{/each}` for consistency  

🟡 **WORTH ADDRESSING**: No image fallback  
   Where: Recipes.svelte line 37, `<img src={recipe.image} ...`  
   Trade-off: Assumes all recipes have valid images  
   Consider: Add an `onerror` handler or placeholder image for missing/broken images  

🟢 **MINOR**: Recipe interface could be shared  
   Where: Recipes.svelte defines its own Recipe type; Favorites has similar inline  
   Consider: If you add more recipe-related components, extract to `src/types/recipe.ts`  
   But: Premature abstraction is worse than duplication. Wait until you have 3+ uses.

---

**You're ready to move on when you can confidently answer the check-in questions.**

---

