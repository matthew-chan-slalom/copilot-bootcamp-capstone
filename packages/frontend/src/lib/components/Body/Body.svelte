<script lang="ts">
	import { onMount, getContext } from "svelte";
	import Favorites from "$lib/components/Body/Favorites.svelte";
	import Recipes from "$lib/components/Body/Recipes.svelte";
	import type { Recipe } from "$lib/models/recipes.interface";

	let items = $state<Recipe[]>([]);
	let favorites = $state<Recipe[]>([]);

	const searchContext = getContext<{ value: string }>("searchQuery");

	// Filtered items based on search query
	let filteredItems = $derived(
		searchContext.value.trim() === ""
			? items
			: items.filter((recipe) =>
					recipe.title.toLowerCase().includes(searchContext.value.toLowerCase())
			  )
	);

	// Filtered favorites based on search query
	let filteredFavorites = $derived(
		searchContext.value.trim() === ""
			? favorites
			: favorites.filter((recipe) =>
					recipe.title.toLowerCase().includes(searchContext.value.toLowerCase())
			  )
	);

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

	function handleFavoritesConsider(e: CustomEvent) {
		favorites = e.detail.items;
	}

	function handleFavoritesFinalize(e: CustomEvent) {
		favorites = e.detail.items;
	}
</script>

<main class="flex-1 p-6 flex flex-col gap-10">
	<Favorites favorites={filteredFavorites} handleConsider={handleFavoritesConsider} handleFinalize={handleFavoritesFinalize} />
	<Recipes items={filteredItems} {handleConsider} {handleFinalize} />
</main>
