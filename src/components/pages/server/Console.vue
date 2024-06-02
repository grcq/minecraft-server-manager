<template>
    <div id="console" class="flex-column">
        <h1>Server</h1>
        <div class="server-data">
            <div class="terminal">
                <div class="terminal-output">
                    <!-- terminal output -->
                </div>
                <div class="terminal-input">
                    <input type="text" placeholder="Enter command" />
                </div>
            </div>
            <div class="server-info">
                <div class="actions">
                    <v-btn color="success" variant="flat" @click="action(id, 'start')">Start</v-btn>
                    <v-btn color="error" variant="flat" @click="action(id, 'stop')">Stop</v-btn>
                    <v-btn color="warning" variant="flat" @click="action(id, 'restart')">Restart</v-btn>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'console-page',
    props: {
        id: String
    },
    async setup(props) {
        const path = window.__TAURI__.path;
        const invoke = window.__TAURI__.invoke;
        const id = props.id;

        const dataDir = await path.appDataDir();

        async function action(id, action) {
            if (action === 'start') {
                invoke("send_message_to_service", {
                    message: "StartServer|" + id + "|" + await path.appDataDir() + "/servers",
                    read: false
                });
            } else if (action === 'stop') {
                invoke("send_message_to_service", {
                    message: "StopServer|" + id,
                    read: false
                });
            } else if (action === 'restart') {
                invoke("send_message_to_service", {
                    message: "StopServer|" + id,
                    read: false
                });
                setTimeout(async () => {
                    invoke("send_message_to_service", {
                        message: "StartServer|" + id + "|" + await path.appDataDir() + "/servers",
                        read: false
                    });
                }, 3000);
            }
        }

        return {
            action
        }
    },
    async mounted() {
        const path = window.__TAURI__.path;
        const invoke = window.__TAURI__.invoke;
        const id = this.id;

        const dataDir = await path.appDataDir();

        var scrollHeight = 0;
        
        let output = [];
        function updateOutput(id) {
            /*const isRunning = await invoke("send_message_to_service", {
                message: "IsRunning|" + id,
            });
            if (!isRunning) {
                output = [];
                document.querySelector(".terminal-output").innerHTML = "";
                return;
            }*/

            const o = invoke("send_message_to_service", {
                message: "GetOutput|" + id,
                read: true
            }).then(() => {
            // only add new lines
                let lines = o.split("\n");
                lines.shift();
                lines.shift();
                lines.shift();

                let i = 0;
                while (lines && lines.length && lines[0] === output[i]) {
                    lines.shift();
                    i++;
                }

                output = output.concat(lines);
                //check if the user is at the bottom of the terminal
                const terminalOutput = document.querySelector(".terminal-output");
                const atBottom = terminalOutput.scrollTop + terminalOutput.clientHeight >= terminalOutput.scrollHeight;

                for (lines of lines) {
                    terminalOutput.innerHTML += `<div class="terminal-line">${lines}</div>`;
                }

                if (atBottom) {
                    terminalOutput.scrollTop = terminalOutput.scrollHeight;
                }
            })
        }

        const input = document.querySelector(".terminal-input input");
        input.addEventListener("keydown", async function(e) {
            if (e.key === "Enter") {
                const command = input.value;
                input.value = "";
                await invoke("send_command", {
                    path: dataDir + "/servers/" + id,
                    command
                });
            }
        });
        
        updateOutput(id);
        //setInterval(() => updateOutput(id), 1000);

        document.querySelector(".terminal-output").addEventListener("scroll", function() {
            if (scrollHeight < this.scrollHeight) {
                scrollHeight = this.scrollHeight;
            }
        });
    }
}
</script>

<style>
.server-data {
    display: flex;
    gap: 1em;
}

.server-data .terminal {
    flex: 1;
}

.server-data .terminal .terminal-output {
    height: 400px;
    overflow-y: auto;
    background-color: rgb(18, 18, 18);
    color: #fff;
    border: solid 1px #333;
    border-radius: 8px 8px 0 0;
}

.server-data .terminal .terminal-output .terminal-line {
    padding: 5px;
    font-size: 14px;
    font-family: monospace;
    word-break: break-all;
}

.server-data .terminal .terminal-input {
    padding: 5px;
    background-color: rgb(var(--color-2));
    color: #fff;
    border-radius: 0 0 8px 8px;
}

.server-data .terminal .terminal-input input {
    width: 100%;
    padding: 5px;
    background-color: transparent;
    color: #fff;
    border: none;
    outline: none;
    border-radius: 8px 8px 0 0;
}

.server-data .server-info {
    display: flex;
    flex-direction: column;
    gap: 1em;
}

.server-data .server-info .actions {
    display: flex;
}

.server-data .server-info .actions button {
    margin-right: 10px;
}

.termianl-output .terminal-line {
    user-select: all;
}
</style>