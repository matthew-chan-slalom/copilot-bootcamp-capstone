<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import * as Command from "$lib/components/ui/command";
	import { Plus } from "@lucide/svelte";

	let search = $state("");
	let focused = $state(false);
</script>

<header class="flex flex-col gap-4 border-b px-6 py-4">
	<div class="flex items-center gap-4">
		<Sidebar.Trigger />
		<h1 class="flex-1 text-center text-xl font-semibold">Whisk & Wish</h1>
	</div>
	<div class="relative w-full">
		<Command.Root class="rounded-full shadow-none border-0">
			<Command.Input placeholder="Search recipes..." bind:value={search} onfocus={() => focused = true} onblur={() => focused = false} />
			{#if focused}
				<Command.List>
					<Command.Empty>
						<Command.Separator />
						<button class="flex w-full items-center gap-2 px-4 py-2 text-sm">
							<Plus class="h-4 w-4" />
							<span>Add a Recipe</span>
						</button>
					</Command.Empty>
				</Command.List>
			{/if}
		</Command.Root>
	</div>
</header>
