<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";

	export let power_action;

	window.addEventListener("keydown", (e) => {
		if (e.key === "Escape") {
			appWindow.hide();
		}
	});
</script>

<main style="height: calc(100vh - var(--padding) * 3);" class="all-center">
	<h1 style="margin: 0;">
		<!-- warning emoji -->
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="24"
			height="24"
			viewBox="0 0 24 24"
			><path
				fill="#dd0"
				d="M4.47 21h15.06c1.54 0 2.5-1.67 1.73-3L13.73 4.99c-.77-1.33-2.69-1.33-3.46 0L2.74 18c-.77 1.33.19 3 1.73 3zM12 14c-.55 0-1-.45-1-1v-2c0-.55.45-1 1-1s1 .45 1 1v2c0 .55-.45 1-1 1zm1 4h-2v-2h2v2z"
			/></svg
		>

		Warning
	</h1>

	<p>
		Do you really want to
		{#if power_action === "shutdown"}
			shutdown
		{:else if power_action === "reboot"}
			restart
		{:else if power_action === "suspend"}
			suspend
		{:else if power_action === "logout"}
			log out of
		{/if} your computer?
	</p>

	<form
		style="display: grid; grid-template-columns: repeat(2, auto); gap: .5em;"
	>
		<button class="btn" on:click={() => invoke(power_action)}>Confirm</button>
		<button class="btn" on:click={() => appWindow.hide()}>Cancel</button>
	</form>
</main>
