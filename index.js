const { app, Tray, Menu } = require('electron');
const si = require('systeminformation');

const convertBytesToGigabytes = (bytes) => Math.ceil(bytes / (1024 ** 3));

const getMeasuredValues = async () => {
    const { main, max } = await si.cpuTemperature();
    const [gpu] = (await si.graphics()).controllers;
    const { total, used } = await si.mem();

    return {
        cpuMainTemp: main,
        cpuMaxTemp: max,
        gpuTemp: gpu.temperatureGpu,
        memoryUsed: convertBytesToGigabytes(used),
        memoryTotal: convertBytesToGigabytes(total)
    };
}

app.whenReady().then(() => {
    const tray = new Tray('./icon.png');

    setInterval(async () => {
        const {
            cpuMainTemp,
            cpuMaxTemp,
            gpuTemp,
            memoryUsed,
            memoryTotal
        } = await getMeasuredValues();
        const contextMenu = Menu.buildFromTemplate([
            { label: `CPU Temp: ${cpuMainTemp} °C`, enabled: false },
            { label: `Max Temp: ${cpuMaxTemp} °C`, enabled: false },
            { label: `GPU Temp: ${gpuTemp} °C`, enabled: false },
            { label: `Memory: ${memoryUsed}GB / ${memoryTotal}GB`, enabled: false },
            { label: 'Exit', role: 'quit' },
        ]);

        tray.setContextMenu(contextMenu);
    }, 2000);

    tray.setToolTip('Hola, bonjour!');
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
})