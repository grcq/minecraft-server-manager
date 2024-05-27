<template>
    <div id="servers">
        <h1>Servers</h1>
        <div class="servers-container">
            <div class="add-server">
                <v-btn prepend-icon="mdi-plus" color="blue">Add</v-btn>
            </div>
            <div id="add-server-popup">
                <v-card class="bg-grey-darken-4">
                    <div class="ma-2">
                        <v-alert id="alert" type="error" elevation="2" icon="mdi-alert" style="display: none" dense></v-alert>
                    </div>
                    <v-card-title>Add Server</v-card-title>
                    <v-card-text>
                        <v-text-field id="server-name" label="Server Name" outlined></v-text-field>
                        <v-text-field id="server-port" label="Server Port" type="number" outlined></v-text-field>
                        <v-text-field id="server-version" label="Server Version" outlined></v-text-field>
                        <v-select id="server-type" label="Server Type" model-value="Vanilla" :items="['Vanilla', 'Paper', 'Sponge', 'Forge', 'BungeeCord', 'Waterfall', 'Velocity']" item-value="first" outlined></v-select>
                        <v-divider></v-divider>
                        <v-text-field id="server-memory" label="Memory" suffix="MB" type="number" outlined></v-text-field>
                    </v-card-text>
                    <v-card-actions class="ms-2">
                        <v-btn id="add-server-add" color="success" variant="flat">Add</v-btn>
                        <v-btn id="add-server-cancel" color="error">Cancel</v-btn>
                    </v-card-actions>
                </v-card>
            </div>
            <div class="server-list">
                <v-card v-for="server of servers" :key="server.id" :id="server.id" @click="showServer(server.id)" class="server bg-grey-darken-4">
                    <v-card-title>{{ server.name }}</v-card-title>
                    <v-card-subtitle>{{ server.type }} {{ server.version }} (ID: {{ server.id }})</v-card-subtitle>
                    <v-card-text>
                        <p>Port: {{ server.port }}</p>
                        <p>Status: Offline</p>
                    </v-card-text> 
                </v-card>
            </div>
        </div>
    </div>
    <div id="server-item">
        <console-page v-if="selectedServer != null" :id="selectedServer" />
    </div>
</template>

<script>
import Console from './server/Console.vue';
import { ref } from 'vue';

export default {
    name: 'servers-page',
    components: {
        'console-page': Console
    },
    async setup() {
        const path = window.__TAURI__.path;
        const invoke = window.__TAURI__.invoke;
        const selectedServer = ref(null);

        function showServer(id) {
            selectedServer.value = id;
        }

        const dataDir = await path.appDataDir();
        const servers = JSON.parse(await invoke("read_file", {
            path: dataDir + "/data/servers.json"
        }));

        return {
            selectedServer,
            servers,
            showServer
        }
    },
    async mounted() {
        const path = window.__TAURI__.path;
        const invoke = window.__TAURI__.invoke;

        const dataDir = await path.appDataDir();
        const servers = JSON.parse(await invoke("read_file", {
            path: dataDir + "/data/servers.json"
        }));

        document.getElementById('add-server-popup').style.display = 'none';
        document.getElementById('add-server-popup').addEventListener('click', function (e) {
            if (e.target.id === 'add-server-popup') {
                document.getElementById('add-server-popup').style.display = 'none';
            }
        });

        document.querySelector('.add-server').addEventListener('click', function () {
            document.getElementById('add-server-popup').style.display = 'block';
        });
        document.getElementById('add-server-cancel').addEventListener('click', function () {
            document.getElementById('add-server-popup').style.display = 'none';
        });

        document.getElementById('add-server-add').addEventListener('click', function () {
            const name = document.getElementById('server-name').value;
            const port = document.getElementById('server-port').value;
            const version = document.getElementById('server-version').value;
            const type = document.getElementById('server-type').value;
            const memory = document.getElementById('server-memory').value;

            if (!name || !port || !version || !type || !memory) {
                document.getElementById('alert').style.display = 'block';
                document.getElementById('alert').innerText = 'Missing fields. Please fill out all fields.';
                return;
            }

            if (port < 1 || port > 65535) {
                document.getElementById('alert').style.display = 'block';
                document.getElementById('alert').innerText = 'Port must be between 1 and 65535';
                return;
            }

            if (memory < 128) {
                document.getElementById('alert').style.display = 'block';
                document.getElementById('alert').innerText = 'Memory must be greater than 128 MB';
                return;
            }

            document.getElementById('add-server-popup').style.display = 'none';

            createServer(name, port, version, type, memory);
        });

        async function createServer(name, port, version, type, memory) {
            const id = Math.random().toString(36).substring(7);

            servers.push({
                id: id,
                name: name,
                port: port,
                version: version,
                type: type
            });

            await invoke("write_file", {
                path: dataDir + "/data/servers.json",
                content: JSON.stringify(servers)
            });

            await invoke("create_dir_if_not_exists", {
                path: dataDir + "/servers"
            })
            
            await invoke("install_server", {
                path: dataDir + "/servers/" + id,
                serverData: {
                    name: name,
                    port: port,
                    version: version,
                    type: type,
                    memory: memory
                }
            });

            addServer(id, name, port, version, type);
        }

        function addServer(id, name, port, version, type) {
            const server = document.createElement('div');
            server.classList.add('server');
            server.innerHTML = `
                <div id="${id}" class="server v-card v-theme--light v-card--density-default v-card--variant-elevated bg-grey-darken-4">
                    <div class="v-card-title">${name}</div>
                    <div class="v-card-subtitle">${type} ${version} (ID: ${id})</div>
                    <div class="v-card-text">
                        <p>Port: ${port}</p>
                        <p>Status: Offline</p>
                    </div> 
                </div>
            `;

            document.querySelector('.server-list').appendChild(server);
        }

        document.querySelectorAll('.server').forEach(element => {
            element.addEventListener('click', function () {
                showServer("console");
            });
        });

        async function showServer(display) {
            const home = document.getElementById('home');
            const servers = document.getElementById('servers');
            const settings = document.getElementById('settings');
            const console = document.getElementById('console');

            home.style.display = 'none';
            servers.style.display = 'none';
            settings.style.display = 'none';
            if (console) console.style.display = display === 'console' ? 'flex' : 'none';

            document.getElementById("server-section").style.display = 'block';

            document.querySelector('.sidebar-item.active').classList.remove('active');
            if (display === 'console') {
                document.getElementById('console-btn').classList.add('active');
            }
        }
    }
}
</script>

<style>
input[type=number]::-webkit-inner-spin-button,
input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.servers-container {
    display: flex;
    flex-direction: column;
    width: 100%;
}

.add-server {
    margin-bottom: 20px;
    margin-right: 0;
    margin-left: auto;
}

.server-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 20px;
}

.server {
    cursor: pointer;
}

#add-server-popup {
    display: none;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 100;
}

#add-server-popup > div {
    margin: auto;
    margin-top: 100px;
    width: 50%;
}
</style>