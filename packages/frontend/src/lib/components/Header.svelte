<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import * as Command from "$lib/components/ui/command";
	import { Plus } from "@lucide/svelte";
	import { getContext } from "svelte";

	let focused = $state(false);

	const openDrawer = getContext<() => void>("openDrawer");
	const searchContext = getContext<{ value: string }>("searchQuery");

	function handleBlur() {
		// Delay blur to allow click events to fire first
		setTimeout(() => {
			focused = false;
		}, 200);
	}
</script>

<header class="sticky top-0 z-10 flex flex-col gap-4 border-b bg-background px-6 py-4">
	<div class="flex items-center gap-4">
		<Sidebar.Trigger />
		<h1 class="flex-1 text-center text-xl font-semibold">Whisk & Wish</h1>
	</div>
	<div class="relative w-full">
		<Command.Root class="rounded-full shadow-none border-0">
			<Command.Input placeholder="Search recipes..." bind:value={searchContext.value} onfocus={() => focused = true} onblur={handleBlur} />
			{#if focused}
				<Command.List>
					<Command.Empty>
						<Command.Separator />
						<button class="flex w-full items-center gap-2 px-4 py-2 text-sm" onclick={openDrawer}>
							<Plus class="h-4 w-4" />
							<span>Add a Recipe</span>
						</button>
					</Command.Empty>
				</Command.List>
			{/if}
		</Command.Root>
	</div>
</header>
