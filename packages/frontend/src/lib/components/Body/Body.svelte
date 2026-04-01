<script lang="ts">
	import recipesData from "../../../resources/recipes.json";
	import Favorites from "$lib/components/Body/Favorites.svelte";
	import Recipes from "$lib/components/Body/Recipes.svelte";

	let items = $state([...recipesData].sort((a, b) => a.title.localeCompare(b.title)));
	let favorites = $state(recipesData.filter((r) => r.isFavorite).sort((a, b) => a.title.localeCompare(b.title)));

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
