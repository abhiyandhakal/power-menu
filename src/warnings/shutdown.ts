import App from "../Warnings.svelte";
import "/src/style.scss";

const app = new App({
	target: document.getElementById("app"),
	props: {
		power_action: "shutdown",
	},
});

export default app;
