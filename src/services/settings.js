const invoke = window.__TAURI__.invoke;
const dataDir = await window.__TAURI__.path.appDataDir();

async function getSettings() {
    return JSON.parse(await invoke('read_file', {
        path: dataDir + '/data/settings.json'
    }));
}

async function getSetting(key, defaultValue = false) {
    const settings = await getSettings();
    return settings[key] === undefined ? defaultValue : settings[key];
}

async function updateSettings(content) {
    invoke('write_file', {
        path: dataDir + '/data/settings.json',
        content: JSON.stringify(content)
    });
}

export {
    getSettings,
    getSetting,
    updateSettings
}