<script lang="ts">
	import { Router, Route } from "svelte-routing";
	import Settings from "./routes/Settings.svelte";
	import Homepage from "./routes/Homepage.svelte";



	export let url = "";
</script>

<Router {url}>
	<Route path="settings" component={Settings} />
	<Route path="/" component={Homepage} />
</Router>
