import App from "./App.svelte";
import "./style.scss";
import { WebviewWindow } from "@tauri-apps/api/window";

// multiple windows
new WebviewWindow("settings", {
	url: "/settings.html",
	title: "Power Menu - Settings",
});

new WebviewWindow("shutdown_warning", {
	url: "/warnings/shutdown.html",
	height: 250,
	width: 500,
	title: "Power Menu - Shutdown Warning",
});

new WebviewWindow("suspend_warning", {
	url: "/warnings/suspend.html",
	height: 250,
	width: 500,
	title: "Power Menu - Suspend Warning",
});

new WebviewWindow("logout_warning", {
	url: "/warnings/logout.html",
	height: 250,
	width: 500,
	title: "Power Menu - Log Out Warning",
});

new WebviewWindow("reboot_warning", {
	url: "/warnings/reboot.html",
	height: 250,
	width: 500,
	title: "Power Menu - Restart Warning",
});

const app = new App({
	target: document.getElementById("app"),
});

export default app;
