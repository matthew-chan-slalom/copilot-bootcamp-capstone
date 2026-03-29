<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import AppHeader from "$lib/components/Header.svelte";
	import Body from "$lib/components/Body/Body.svelte";
</script>

<Sidebar.Inset>
	<AppHeader />
	<Body />
</Sidebar.Inset>
