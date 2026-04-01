<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import MainContent from "$lib/components/Main.svelte";
	import AppSidebar from "$lib/components/Sidebar.svelte";
	import Drawer from "$lib/components/Drawer/Drawer.svelte";
	import { setContext } from "svelte";

	let drawerOpen = $state(false);

	setContext("openDrawer", () => {
		drawerOpen = true;
	});
</script>

<Sidebar.Provider>
	<AppSidebar />
	<MainContent />
	<Drawer bind:open={drawerOpen} />
</Sidebar.Provider>
