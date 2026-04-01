<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { Home, Settings, Users, Plus, UtensilsCrossed } from "@lucide/svelte";
	import { getContext } from "svelte";

	const openDrawer = getContext<() => void>("openDrawer");
</script>

<Sidebar.Root>
	<Sidebar.Header>
		<h2 class="flex items-center gap-2 px-4 py-2 text-lg font-semibold">
			<UtensilsCrossed class="h-5 w-5" />
			Whisk & Wish
		</h2>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
						<Users class="h-4 w-4" />
						<span>Users</span>
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
							{#snippet child({ props })}
								<a href="/" {...props}>
									<Home class="h-4 w-4" />
									<span>Home</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
					<Sidebar.Separator />
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Recipes</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton onclick={openDrawer}>
						<Plus class="h-4 w-4" />
						<span>Add a Recipe</span>
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupLabel>General</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
						<Settings class="h-4 w-4" />
						<span>Settings</span>
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<p class="px-4 py-2 text-sm text-muted-foreground">© 2026</p>
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
