<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import MainContent from "$lib/components/Main.svelte";
	import AppSidebar from "$lib/components/Sidebar.svelte";
	import Drawer from "$lib/components/Drawer/Drawer.svelte";
	import { setContext } from "svelte";

	let drawerOpen = $state(false);
	let searchQuery = $state("");

	setContext("openDrawer", () => {
		drawerOpen = true;
	});

	setContext("searchQuery", {
		get value() {
			return searchQuery;
		},
		set value(newValue: string) {
			searchQuery = newValue;
		}
	});
</script>

<Sidebar.Provider>
	<AppSidebar />
	<MainContent />
	<Drawer bind:open={drawerOpen} />
</Sidebar.Provider>
