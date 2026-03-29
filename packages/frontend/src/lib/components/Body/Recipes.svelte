<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { ChefHat } from "@lucide/svelte";
	import { dndzone } from "svelte-dnd-action";

	interface Recipe {
		id: number;
		title: string;
		description: string;
		image: string;
		isFavorite: boolean;
	}

	interface Props {
		items: Recipe[];
		handleConsider: (e: CustomEvent) => void;
		handleFinalize: (e: CustomEvent) => void;
	}

	let { items, handleConsider, handleFinalize }: Props = $props();
</script>

<section>
	<h2 class="flex items-center gap-2 text-lg font-semibold mb-4">
		<ChefHat class="h-5 w-5" />
		Recipes
	</h2>
	<div
		class="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4"
		use:dndzone={{ items, flipDurationMs: 200 }}
		onconsider={handleConsider}
		onfinalize={handleFinalize}
	>
		{#each items as recipe (recipe.id)}
			<Card.Root class="overflow-hidden cursor-grab active:cursor-grabbing">
				<img src={recipe.image} alt={recipe.title} class="w-full object-cover h-40 bg-muted" loading="lazy" />
				<Card.Header>
					<Card.Title>{recipe.title}</Card.Title>
					<Card.Description>{recipe.description}</Card.Description>
				</Card.Header>
			</Card.Root>
		{/each}
	</div>
</section>
