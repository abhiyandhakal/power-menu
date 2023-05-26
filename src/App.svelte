<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";

	const list: {
		id: number;
		content: string;
		func_name: string;
	}[] = [
		{ id: 0, content: "Shutdown", func_name: "shutdown" },
		{ id: 1, content: "Suspend", func_name: "suspend" },
		{ id: 2, content: "Restart", func_name: "restart" },
		{ id: 3, content: "Log Out", func_name: "logout" },
	];

	$: current = 0;

	async function handleclick(current_number: number) {
		await invoke(list[current_number].func_name);
	}

	window.addEventListener("keydown", async (e: KeyboardEvent) => {
		// movd down
		if (e.key === "j" || e.key === "ArrowDown") {
			current < list.length - 1 ? current++ : (current = 0);
		}
		// move up
		else if (e.key === "k" || e.key === "ArrowUp") {
			current > 0 ? current-- : (current = list.length - 1);
		}
		// execute command
		else if (e.key === "Enter") {
			handleclick(current);
		}
		// quit
		else if (e.key === "Escape") {
			await appWindow.close();
		}
	});
</script>

<h1>Power Menu</h1>

{#each list as { id, content }}
	<button
		on:click={() => handleclick(id)}
		class={`${id === current ? "active" : ""}`}>{content}</button
	>
{/each}
