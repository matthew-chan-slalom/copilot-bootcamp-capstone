<script lang="ts">
	import { onMount } from "svelte";
	import Favorites from "$lib/components/Body/Favorites.svelte";
	import Recipes from "$lib/components/Body/Recipes.svelte";
	import type { Recipe } from "$lib/models/recipes.interface";

	let items = $state<Recipe[]>([]);
	let favorites = $state<Recipe[]>([]);

	onMount(async () => {
		try {
			const response = await fetch("http://127.0.0.1:3001/recipes");
			if (!response.ok) throw new Error("Failed to fetch recipes");
			
			const recipesData: Recipe[] = await response.json();
			items = [...recipesData].sort((a, b) => a.title.localeCompare(b.title));
			favorites = recipesData.filter((r) => r.isFavorite).sort((a, b) => a.title.localeCompare(b.title));
		} catch (error) {
			console.error("Error loading recipes:", error);
		}
	});

	function handleConsider(e: CustomEvent) {
		items = e.detail.items;
	}

	function handleFinalize(e: CustomEvent) {
		items = e.detail.items;
	}
</script>

<main class="flex-1 p-6 flex flex-col gap-10">
	<Favorites bind:favorites={favorites} />
	<Recipes {items} {handleConsider} {handleFinalize} />
</main>
