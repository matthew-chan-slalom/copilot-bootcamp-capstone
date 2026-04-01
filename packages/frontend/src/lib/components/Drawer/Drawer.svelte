<script lang="ts">
	import * as Sheet from "$lib/components/ui/sheet";
	import { X } from "@lucide/svelte";

	interface Props {
		open?: boolean;
		onOpenChange?: (open: boolean) => void;
	}

	let { open = $bindable(false), onOpenChange }: Props = $props();

	function handleOpenChange(isOpen: boolean) {
		open = isOpen;
		onOpenChange?.(isOpen);
	}
</script>

<Sheet.Root {open} onOpenChange={handleOpenChange}>
	<Sheet.Content side="right" class="w-full sm:max-w-md p-6">
		<Sheet.Header class="-ml-6">
			<Sheet.Title>Add a Recipe</Sheet.Title>
			<Sheet.Description>
				Create a new recipe by filling out the form below.
			</Sheet.Description>
		</Sheet.Header>
		
		<div class="flex flex-col gap-4 py-4">
			<div class="flex flex-col gap-2">
				<label for="title" class="text-sm font-medium">Title</label>
				<input
					id="title"
					type="text"
					class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
					placeholder="Enter recipe title"
				/>
			</div>
			
			<div class="flex flex-col gap-2">
				<label for="description" class="text-sm font-medium">Description</label>
				<textarea
					id="description"
					class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
					placeholder="Enter recipe description"
				></textarea>
			</div>
		</div>

		<Sheet.Footer class="flex gap-2">
			<button
				type="button"
				onclick={() => handleOpenChange(false)}
				class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2"
			>
				Cancel
			</button>
			<button
				type="submit"
				class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
			>
				Add Recipe
			</button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>
