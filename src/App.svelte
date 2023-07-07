<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
	import type { Config } from "./vite-env";
	import { onMount } from "svelte";

	let config: Config = {
		warn: true,
	};

	const list: {
		id: number;
		content: string;
		func_name: string;
	}[] = [
		{ id: 0, content: "Shutdown", func_name: "shutdown" },
		{ id: 1, content: "Suspend", func_name: "suspend" },
		{ id: 2, content: "Restart", func_name: "reboot" },
		{ id: 3, content: "Log Out", func_name: "logout" },
	];

	async function power_action(current_number: number) {
		const res = await invoke("get_config");

		config = res as Config;

		if (config.warn) {
			WebviewWindow.getByLabel(
				`${list[current_number].func_name}_warning`
			).show();
			appWindow.hide();
		} else {
			invoke(list[current_number].func_name);
			appWindow.hide();
		}
	}

	let power_btns: NodeListOf<HTMLElement>;

	// startup focus
	onMount(() => {
		document.getElementById("power-btn-shutdown").focus();

		power_btns = document.querySelectorAll("[data-power-btn]");
	});

	// keydown event listener
	document.addEventListener("keydown", (e) => {
		if (e.key === "Escape" || e.key.toLowerCase() === "q") {
			appWindow.hide();
		}

		let power_btns_array = Array.from(power_btns);

		if (
			e.key === "ArrowDown" ||
			e.key.toLowerCase() === "j" ||
			e.key.toLowerCase() === "l"
		) {
			const active_btn = power_btns_array.find(
				(item) => item.id === document.activeElement.id
			);

			if (active_btn === undefined) {
				power_btns_array[0].focus();
			} else {
				power_btns_array[
					(parseInt(active_btn.dataset.index) + 1) % power_btns_array.length
				].focus();
			}
		}

		if (
			e.key === "ArrowUp" ||
			e.key.toLowerCase() === "k" ||
			e.key.toLowerCase() === "h"
		) {
			const active_btn = power_btns_array.find(
				(item) => item.id === document.activeElement.id
			);

			if (active_btn === undefined) {
				power_btns_array[0].focus();
			} else {
				const prev_index =
					parseInt(active_btn.dataset.index) - 1 < 0
						? power_btns_array.length - 1
						: parseInt(active_btn.dataset.index) - 1;

				power_btns_array[prev_index].focus();
			}
		}
	});
</script>

<h1>Power Menu</h1>

<button
	class="settings"
	on:click={() => {
		WebviewWindow.getByLabel("settings").show();
	}}></button
>

{#each list as { id, content, func_name } (id)}
	<button
		class="big-btn"
		id={`power-btn-${func_name}`}
		data-power-btn
		data-index={id}
		on:click={() => power_action(id)}
	>
		{content}
	</button>
{/each}
