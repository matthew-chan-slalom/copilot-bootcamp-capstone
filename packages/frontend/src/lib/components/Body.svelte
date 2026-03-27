<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { ChefHat, Heart } from "@lucide/svelte";
	import recipesData from "../../resources/recipes.json";
	import { dndzone } from "svelte-dnd-action";

	let items = $state([...recipesData].sort((a, b) => a.title.localeCompare(b.title)));
	let favorites = $state(recipesData.filter((r) => r.isFavorite).sort((a, b) => a.title.localeCompare(b.title)));

	function handleConsider(e: CustomEvent) {
		items = e.detail.items;
	}

	function handleFinalize(e: CustomEvent) {
		items = e.detail.items;
	}

	function handleFavConsider(e: CustomEvent) {
		favorites = e.detail.items;
	}

	function handleFavFinalize(e: CustomEvent) {
		favorites = e.detail.items;
	}
</script>

<main class="flex-1 p-6 flex flex-col gap-10">
	<section>
		<h2 class="flex items-center gap-2 text-lg font-semibold mb-4">
			<Heart class="h-5 w-5" />
			Favorites
		</h2>
		<div
			class="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 min-h-32 rounded-lg border-2 border-dashed border-muted p-4"
			use:dndzone={{ items: favorites, flipDurationMs: 200 }}
			onconsider={handleFavConsider}
			onfinalize={handleFavFinalize}
		>
			{#each favorites as recipe (recipe.id)}
				<Card.Root class="overflow-hidden cursor-grab active:cursor-grabbing">
					<img src={recipe.image} alt={recipe.title} class="w-full object-cover h-40 bg-muted" loading="lazy" />
					<Card.Header>
						<Card.Title>{recipe.title}</Card.Title>
						<Card.Description>{recipe.description}</Card.Description>
					</Card.Header>
				</Card.Root>
			{:else}
				<p class="text-sm text-muted-foreground col-span-full self-center text-center">Drag recipes here to add to favorites</p>
			{/each}
		</div>
	</section>

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
</main>
