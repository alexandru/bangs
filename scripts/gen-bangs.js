#!/usr/bin/env node

const https = require('https');

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
    let kotlinCode = `\nval bangsData = arrayOf(\n`;
    let currentMap = '  mapOf(\n';
    let mapCount = 0;

    bangs.forEach((bang, index) => {
        const escapedT = bang.t.replace(/"/g, '\\"').replace(/\$/g, '\\$');
        const escapedU = bang.u.replace(/"/g, '\\"').replace(/\$/g, '\\$');
        currentMap += `    "${escapedT}" to "${escapedU}"`;

        if (index < bangs.length - 1) {
            currentMap += ',\n';
        }

        if ((index + 1) % 8192 === 0 || index === bangs.length - 1) {
            currentMap += '\n  )';
            if (index < bangs.length - 1) {
                currentMap += ',';
            }
            currentMap += '\n';
            kotlinCode += currentMap;
            currentMap = '  mapOf(\n';
            mapCount++;
        }
    });

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
