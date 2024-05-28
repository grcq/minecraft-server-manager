<template>
    <div id="home">
        <h1>Welcome to MSM!</h1>
        <v-card class="bg-grey-darken-4">
            <v-card-title>Information</v-card-title>
            <v-card-text>
                <p>abc</p>
            </v-card-text>
        </v-card>
    </div>
</template>

<script>
import { options } from '../../services/rpc.js'
import { getSetting } from '../../services/settings.js';

const invoke = window.__TAURI__.invoke;


export default {
    name: 'home-page'
}
</script>