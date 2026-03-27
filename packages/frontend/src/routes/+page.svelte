<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import MainContent from "$lib/components/Main.svelte";
	import AppSidebar from "$lib/components/Sidebar.svelte";
</script>

<Sidebar.Provider>
	<AppSidebar />
	<MainContent />
</Sidebar.Provider>
