<template>
    <div id="files" class="flex-column">
        <h1>Files</h1>

        <div class="files-path">
            <div class="separator">/</div>
            <div class="path-name" @click="openFile(null)">container</div>
            <div class="separator" v-for="p in openedFileSplit">/</div>
            <div class="path-name" v-for="p in openedFileSplit">{{ p }}</div>
        </div>
        <div class="files">
            <div class="file" v-for="file in files" :key="file">
                <div class="file-name" @click="openFile(file)">{{ file }}</div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref } from 'vue';

export default {
    name: 'files-page',
    props: {
        id: String
    },
    async setup(props) {
        const path = window.__TAURI__.path;
        const invoke = window.__TAURI__.invoke;
        const id = props.id;

        const openedFile = ref("");
        const openedFileSplit = ref([]);

        if (openedFile.value != "") {
            openedFileSplit.value = openedFile.value.split("/");
        }

        function openFile(file) {
            openedFile.value = file;
            openedFileSplit.value = file.split("/");
        }

        const dataDir = await path.appDataDir();
        const serverDir = dataDir + "/servers/" + id;

        let filesArray = await invoke("read_dir", {
            path: serverDir
        });
        // map files to remove dataDir
        filesArray = filesArray.map(f => f.replace(serverDir + "\\", ""));
        const files = ref(filesArray);

        return {
            files,
            openedFile,
            openedFileSplit,
            openFile
        }
    }
}
</script>

<style>
    .files-path {
        display: flex;
        gap: 8px;
        align-items: center;
        margin-bottom: 8px;
    }

    .path-name {
        font-size: 14px;
        color: rgba(255, 255, 255, 0.8);       
    }

    .files {
        display: flex;
        flex-direction: column;
        background-color: rgb(var(--color-2));
        border-radius: 5px;
    }

    .file {
        cursor: pointer;
        padding: 8px;
        border-bottom: 1px solid rgba(var(--color-1), 0.2);

    }

    .file:hover {
        background-color: rgba(var(--color-2), 0.7);
    }

    .file-name {
        font-size: 14px;
    }
</style>