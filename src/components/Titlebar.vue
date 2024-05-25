<template>
    <div data-tauri-drag-region class="titlebar-container">
        <div data-tauri-drag-region class="titlebar">
            <div class="titlebar-options">
                <div class="options-menu">
                    <div @click="showMenu" class="options-menu-toggler">
                        <span class="line"></span>
                        <span class="line"></span>
                        <span class="line"></span>
                    </div>
                    <div class="options-menu-items">
                        <div class="options-menu-item">New Server</div>
                        <div class="options-menu-item">Servers</div>
                        <div class="options-menu-item">Item 3</div>
                    </div>
                </div>
            </div>
            <div class="titlebar-title">MSM</div>
            <div class="titlebar-options">
                <div @click="minimize" class="titlebar-option-r titlebar-minimize mx-1"></div>
                <div @click="maximize" class="titlebar-option-r titlebar-maximize mx-1"></div>
                <div @click="close" class="titlebar-option-r titlebar-close mx-1"></div>
            </div>
        </div>
    </div>
</template>

<script>
import { appWindow } from '@tauri-apps/api/window'

export default {
    name: 'title-bar',
    methods: {
        minimize() {
            appWindow.minimize()
        },
        maximize() {
            appWindow.toggleMaximize()
        },
        close() {
            appWindow.close()
        },
        showMenu() {
            const items = document.querySelector('.options-menu-items');
            if (items.classList.contains('show')) {
                items.classList.remove('show');
            } else {
                items.classList.add('show');
            }
        }
    },
}
</script>

<style>
.titlebar-container {
    padding: 10px;
    color: white;
    height: 10svh;
}

.titlebar {
    padding: 10px;
    border-radius: 8px;
    display: flex;
    justify-content: space-between;
    background-color: rgb(var(--color-2));
    align-items: center;
}

.line {
    width: 20px;
    height: 2px;
    background-color: white;
    margin: 2px 0;
}

.options-menu {
    position: relative;
    display: flex;
    align-items: center;
    cursor: pointer;
}

.options-menu-toggler {
    padding: 10px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border-radius: 8px;
}

.options-menu-toggler:hover {
    background-color: rgba(255,255,255,0.1);
}

.options-menu-items {
    position: absolute;
    display: none;
    flex-direction: column;
    background-color: rgb(var(--color-2));
    border-radius: 0 0 8px 8px;
    top: 5svh;
    left: -1svh;
    min-width: 150px;
    width: auto;
    font-weight: 600;
    z-index: 1000;
}

.options-menu-items.show {
    display: flex;
}

.options-menu-item {
    padding: 8px;
    cursor: pointer;
    border-radius: 5px;
}

.options-menu-item:hover {
    background-color: rgba(255,255,255,0.1);
}

.titlebar-options {
    display: flex;
}

.titlebar-title {
    font-size: 1.5rem;
    font-weight: bold;
}

.titlebar-option-r {
    position: relative;
    width: 15px;
    height: 15px;
    border-radius: 50%;
    cursor: pointer;
}

.titlebar-option-r:hover::before {
    position: absolute;
    display: flex;
    justify-content: center;
    color: rgba(0,0,0,0.5);
    right: 0;
    left: 0;
    top: 0;
    bottom: 0;
    font-size: 11px;
}

.titlebar-minimize {
    background-color: #ffcc00;
}

.titlebar-minimize:hover {
    background-color: #d6ab00;
}

.titlebar-minimize:hover::before {
    content: '-';
}

.titlebar-maximize {
    background-color: #33cc33;
}

.titlebar-maximize:hover {
    background-color: #2db82d;
}

.titlebar-maximize:hover::before {
    content: '+';
}

.titlebar-close {
    background-color: #ff6666;
}

.titlebar-close:hover {
    background-color: #e65c5c;
}

.titlebar-close:hover::before {
    content: 'x';
}

</style>