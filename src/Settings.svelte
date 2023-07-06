<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";

	interface Config {
		warn: boolean;
	}

	let config: Config;

	(async function () {
		const res = await invoke("get_config");
		config = res as Config;
	})();

	let warn = true;

	$: () => {
		const to_send = { warn };

		appWindow.emit("settings", {
			message: to_send,
		});
	};
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
			bind:value={warn}
		/>
		<label for="warn" class="toggle-label" />
	</div>
</div>
