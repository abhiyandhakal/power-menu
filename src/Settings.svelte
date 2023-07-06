<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";

	(async function () {
		console.log(await invoke("get_config"));
	})();

	let warn = false;

	function emit() {
		appWindow.emit("settings", {
			message: "hello world from settings frontend",
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
			on:click={emit}
		/>
		<label for="warn" class="toggle-label" />
	</div>
</div>

<style lang="scss">
	.toggle-container {
		display: flex;
		justify-content: space-between;
		font-size: 1.2rem;
	}
</style>
