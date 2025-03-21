#!/usr/bin/env node

const https = require('https');

const chars = [
    '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y',
    'z'
]

// Function to download the file
function downloadFile(url) {
    return new Promise((resolve, reject) => {
        https.get(url, (res) => {
            let data = '';

            res.on('data', (chunk) => {
                data += chunk;
            });

            res.on('end', () => {
                resolve(data);
            });

            res.on('error', (err) => {
                reject(err);
            });
        });
    });
}

// Function to process the downloaded data and generate Kotlin code
function processData(data) {
    const bangs = JSON.parse(data.substring(data.indexOf('['), data.lastIndexOf(']') + 1));
    let perChar = {}

    for (let index = 0; index < bangs.length; index++) {
        const bang = bangs[index];
        const ch = bang.t.charAt(0).toLowerCase();
        if (!/[a-z0-9]/.test(ch)) {
            continue;
        }
        if (!perChar[ch]) {
            perChar[ch] = []
        }
        perChar[ch].push([bang.t, bang.u])
        // perChar[ch] = [bang.t, bang.u]
        // const escapedT = bang.t.replace(/"/g, '\\"').replace(/\$/g, '\\$');
        // const escapedU = bang.u.replace(/"/g, '\\"').replace(/\$/g, '\\$');
        // currentMap += `    "${escapedT}" to "${escapedU}"`;
        //
        // if (index < bangs.length - 1) {
        //     currentMap += ',\n';
        // }
        //
        // if ((index + 1) % 8192 === 0 || index === bangs.length - 1) {
        //     currentMap += '\n  )';
        //     if (index < bangs.length - 1) {
        //         currentMap += ',';
        //     }
        //     currentMap += '\n';
        //     kotlinCode += currentMap;
        //     currentMap = '  mapOf(\n';
        //     mapCount++;
        // }
    }

    let kotlinCode = `\nval bangsData = arrayOf(\n`;

    for (let index = 0; index < chars.length; index++) {
        const ch = chars[index];
        const bangs = perChar[ch];

        let bangsData = `  arrayOf(\n`;
        for (let i = 0; i < bangs.length; i++) {
            const bang = bangs[i];
            const escapedT = bang[0].replace(/"/g, '\\"').replace(/\$/g, '\\$');
            const escapedU = bang[1].replace(/"/g, '\\"').replace(/\$/g, '\\$');
            bangsData += `    "${escapedT}" to "${escapedU}"`;
            if (i < bangs.length - 1) {
                bangsData += ',\n';
            }
        }
        bangsData += '\n  )';

        kotlinCode += bangsData;
        if (index < chars.length - 1) {
            kotlinCode += ',\n';
        }
    }
    kotlinCode += '\n)\n';
    return kotlinCode;
}

// Main function to download and process the file
async function main() {
    const url = 'https://duckduckgo.com/bang.js';

    try {
        const data = await downloadFile(url);
        const kotlinCode = processData(data);
        console.log(kotlinCode);
    } catch (error) {
        console.error('Error downloading or processing the file:', error);
    }
}

// Run the main function
main();
