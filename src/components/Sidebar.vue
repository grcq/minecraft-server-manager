<template>
    <div class="sidebar-container">
        <aside class="sidebar">
            <div class="sidebar-header">
                <div class="sidebar-header-title">Welcome!</div>
            </div>
            <div class="sidebar-separator"></div>
            <div class="sidebar-content">
                <div class="sidebar-items">
                    <div class="sidebar-items-section">
                        <div id="home-btn" class="sidebar-item active">Home</div>
                        <div id="servers-btn" class="sidebar-item">Servers</div>
                        <div id="settings-btn" class="sidebar-item">Settings</div>
                    </div>
                    <div class="sidebar-items-section" id="server-section">
                        <div class="sidebar-content-separator"></div>
                        <div id="console-btn" class="sidebar-item">Console</div>
                        <div class="sidebar-item">Files</div>
                        <div class="sidebar-item">Options</div>
                        <div class="sidebar-item">Backup</div>
                        <div class="sidebar-item">Logs</div>
                    </div>
                </div>
            </div>
        </aside>
    </div>
</template>

<script>
export default {
    name: 'side-bar',
    async mounted() {
        const home = document.getElementById('home');
        const servers = document.getElementById('servers');
        const settings = document.getElementById('settings');
        const console = document.getElementById('console');

        switchDisplay('home');

        document.getElementById('home-btn').addEventListener('click', function () {
            document.querySelector('.sidebar-item.active').classList.remove('active');
            this.classList.add('active');

            switchDisplay('home');
        });

        document.getElementById('servers-btn').addEventListener('click', function () {
            document.querySelector('.sidebar-item.active').classList.remove('active');
            this.classList.add('active');

            switchDisplay('servers');
        });

        document.getElementById('settings-btn').addEventListener('click', function () {
            document.querySelector('.sidebar-item.active').classList.remove('active');
            this.classList.add('active');

            switchDisplay('settings');
        });

        /*console.addEventListener('click', () => {
            document.querySelector('.sidebar-item.active').classList.remove('active');
            console.classList.add('active');

            switchDisplay('console');
        });*/

        function switchDisplay(name) {
            home.style.display = (name === 'home' ? 'flex' : 'none');
            servers.style.display = (name === 'servers' ? 'flex' : 'none');
            settings.style.display = (name === 'settings' ? 'flex' : 'none');
            //console.style.display = (name === 'console' ? 'flex' : 'none');
        }
    }
}
</script>

<style>
.sidebar-container {
    width: 250px;
    min-height: 88svh;
    padding: 12px;
}

.sidebar {
    height: 100%;
    background-color: rgb(var(--color-2));
    border-radius: 8px;
}

.sidebar-header {
    padding: 10px;
    padding-top: 20px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.sidebar-header-title {
    color: white;
    font-size: 20px;
}

.sidebar-separator {
    height: 4px;
    background-color: rgb(var(--primary));
    margin: 10px;
}

.sidebar-content-separator {
    height: 2px;
    background-color: rgb(var(--primary));
    margin: 5px 20px;
}

.sidebar-content {
    padding: 10px;
}

.sidebar-items, .sidebar-items-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

#server-section {
    display: none;
}

#server-section.show {
    display: flex;
}

.sidebar-item {
    padding: 10px;
    border-radius: 8px;
    color: white;
    cursor: pointer;
    transition: ease-in-out 0.15s;
}

.sidebar-item:hover {
    background-color: rgba(var(--primary), 0.5);
}

.sidebar-item.active {
    border-left: 4px solid rgb(var(--primary));
    border-radius: 0 8px 8px 0;
}
</style>