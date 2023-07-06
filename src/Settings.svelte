<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";
	import type { Config } from "./vite-env";

	let config: Config = {
		warn: true,
	};

	invoke("get_config").then((res) => {
		config = res as Config;

		if (config.warn === undefined) {
			config.warn = true;
		}
	});

	$: {
		appWindow.emit("settings", {
			message: config,
		});
	}
</script>

<h1>Settings</h1>

<div class="toggle-container">
	<label for="warn">Warn before performing power actions.</label>
	<div class="toggle">
		<input
			type="checkbox"
			name="toggle"
			class="toggle-checkbox"
			id="warn"
			bind:checked={config.warn}
		/>
		<label for="warn" class="toggle-label" />
	</div>
</div>
