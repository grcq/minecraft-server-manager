<template>
	<Suspense>
		<div class="container">
			<title-bar />
			<div class="d-flex">
				<side-bar />
				<div class="content">
					<home-page />
					<servers-page />
					<settings-page />
				</div>
			</div>
		</div>
</Suspense>
</template>

<script>
import Titlebar from './components/Titlebar.vue'
import Sidebar from './components/Sidebar.vue'
import Homepage from './components/pages/Homepage.vue'
import Servers from './components/pages/Servers.vue'
import Settings from './components/pages/Settings.vue'

const path = window.__TAURI__.path;
const invoke = window.__TAURI__.invoke;
const dataDir = await path.appDataDir();

await invoke("create_dir_if_not_exists", {
	path: dataDir
});
await invoke("create_dir_if_not_exists", {
	path: dataDir + "/data"
});

await invoke("create_file_if_not_exists", {
	path: dataDir + "/data/servers.json",
	content: "[]"
});

await invoke("create_file_if_not_exists", {
	path: dataDir + "/data/settings.json",
	content: "{}"
});

export default {
	name: 'App',
	components: {
		"title-bar": Titlebar,
		"side-bar": Sidebar,
		"home-page": Homepage,
		"servers-page": Servers,
		"settings-page": Settings
	},
	async mounted() {
		//disableContextMenu();
	}
}
</script>

<style>

:root {
	--accent: 142, 68, 240;
	--primary: 47, 75, 231;
	--color-1: 4, 8, 9;
	--color-2: 32, 32, 32;
}

html, body {
	margin: 0;
	padding: 0;
	scrollbar-width: 0;
	height: 100;
	overflow: hidden;
	user-select: none;
	font-family: Nunito;
	color: white;
}

.content {
	padding: 8px;
	padding-right: 16px;
	width: 100%;
}

.content > div {
	flex-direction: column;
}

::-webkit-scrollbar {
	display: none;
}

.container {
	min-height: 100svh;
	background-color: rgb(var(--color-1));
}

</style>