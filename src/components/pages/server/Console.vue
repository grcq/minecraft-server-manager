<template>
    <div id="server">
        <h1>Server {{ id }}</h1>
    </div>
</template>

<script>
export default {
    name: 'console-page',
    props: {
        id: String
    },
    async mounted() {
        const home = document.getElementById('home');
        const servers = document.getElementById('servers');
        const settings = document.getElementById('settings');
        const console = document.getElementById('console');

        home.style.display = 'none';
        servers.style.display = 'none';
        settings.style.display = 'none';
        console.style.display = 'flex';

        document.getElementById("server-section").style.display = 'block';
    }
}
</script>