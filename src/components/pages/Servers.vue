<template>
    <div id="servers">
        <h1>Servers</h1>
        <div class="servers-container">
            <div class="add-server">
                <v-btn prepend-icon="mdi-plus" color="primary">Add</v-btn>
            </div>
            <div id="add-server-popup">
                <v-card>
                    <v-card-title>Add Server</v-card-title>
                    <v-card-text>
                        <v-text-field label="Server Name" outlined></v-text-field>
                        <v-text-field label="Server IP" outlined></v-text-field>
                        <v-text-field label="Server Port" outlined></v-text-field>
                        <v-text-field label="Server Password" outlined></v-text-field>
                    </v-card-text>
                    <v-card-actions>
                        <v-btn color="primary">Add</v-btn>
                        <v-btn color="error">Cancel</v-btn>
                    </v-card-actions>
                </v-card>
            </div>
            <div class="server-list"></div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'servers-page'
}
</script>

<style>
.servers-container {
    display: flex;
    flex-direction: column;
    align-items: center;
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
}
</style>