import App from "./App.svelte";
import "./style.scss";
import { WebviewWindow } from "@tauri-apps/api/window";

new WebviewWindow("settings", {
	url: "/settings.html",
});

const app = new App({
	target: document.getElementById("app"),
});

export default app;
