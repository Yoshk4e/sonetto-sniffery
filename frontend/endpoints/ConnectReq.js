const WebSocket = require('ws');

async function execute() {
    return new Promise((resolve) => {
        const ws = new WebSocket('ws://127.0.0.1:40510/ws');

        const timeout = setTimeout(() => {
            ws.terminate();
            resolve({ sessionStarted: false });
        }, 2000); // 2 second timeout

        ws.on('open', () => {
            ws.send(JSON.stringify({ cmd: 'ConnectReq' }));
        });

        ws.on('message', (data) => {
            try {
                const msg = JSON.parse(data.toString());
                if (msg.cmd === 'ConnectRsp') {
                    clearTimeout(timeout);
                    ws.terminate();
                    resolve({ sessionStarted: true });
                }
            } catch {
                // Ignore parse errors
            }
        });

        ws.on('error', () => {
            clearTimeout(timeout);
            resolve({ sessionStarted: false });
        });

        ws.on('close', () => {
            clearTimeout(timeout);
            resolve({ sessionStarted: false });
        });
    });
}

module.exports = { execute };
