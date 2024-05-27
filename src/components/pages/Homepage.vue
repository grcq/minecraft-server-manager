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

const invoke = window.__TAURI__.invoke;

const homeRpc = options['home'];
invoke('set_rpc', {
    details: homeRpc['details'],
    largeText: homeRpc['large_text'],
    smallText: homeRpc['small_text'],
    timestamp: Date.now()
})

export default {
    name: 'home-page'
}
</script>