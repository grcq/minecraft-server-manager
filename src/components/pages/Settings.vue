<template>
    <div id="settings">
        <h1>Settings</h1>

        <div class="settings">
            <v-card class="bg-grey-darken-4">
                <v-card-title>General</v-card-title>
                <v-card-text>

                </v-card-text>
            </v-card>

            <v-card class="bg-grey-darken-4">
                <v-card-title>Discord</v-card-title>
                <v-card-text>
                    <v-switch v-model="discordRpc" color="success" label="Rich Presence" @change="updateSettings()"></v-switch>
                </v-card-text>
            </v-card>
        </div>
    </div>
</template>

<script>
import { ref } from 'vue';
import { getSetting, updateSettings as uS } from '../../services/settings.js';

const path = window.__TAURI__.path;
const invoke = window.__TAURI__.invoke;

export default {
    name: 'settings-page',
    async setup() {
        const discordRpc = ref(await getSetting('discordRpc', true));

        function updateSettings() {
            const settings = {
                discordRpc: discordRpc.value
            }

            invoke('toggle_rpc', {
                toggle: discordRpc.value
            })

            uS(settings);
        }

        return {
            updateSettings,
            discordRpc
        }
    }
}
</script>

<style>
    .settings {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1rem;
    }
</style>