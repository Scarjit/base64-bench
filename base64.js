const benchInputs = [
    'Hello, World!',
    'Hello, World!'.repeat(100),
    'Hello, World!'.repeat(1000),
    'Hello, World!'.repeat(10000),
    'Hello, World!'.repeat(100000),
    'Hello Unicode! 😊',
];


/*
    Basically Native Base64 encoding and decoding functions
*/

function encode1(str) {
    return btoa(encodeURIComponent(str).replace(/%([0-9A-F]{2})/g,
        function toSolidBytes(match, p1) {
            return String.fromCharCode('0x' + p1);
        }));
}

function decode1(base64) {
    return decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));
}

/*
    Using TextEnc/Decoder
 */

const encodeTable = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
const decodeTable = new Array(256);
for (let i = 0; i < 64; i++) {
    decodeTable[encodeTable.charCodeAt(i)] = i;
}

function encode2(str) {
    const bytes = new TextEncoder().encode(str);
    let output = '';
    for (let i = 0; i < bytes.length; i += 3) {
        const a = bytes[i];
        const b = bytes[i + 1];
        const c = bytes[i + 2];
        output += encodeTable[a >> 2];
        output += encodeTable[((a & 0x3) << 4) | (b >> 4)];
        output += encodeTable[((b & 0xF) << 2) | (c >> 6)];
        output += encodeTable[c & 0x3F];
    }
    switch (bytes.length % 3) {
        case 1:
            output = output.slice(0, -2) + '==';
            break;
        case 2:
            output = output.slice(0, -1) + '=';
            break;
    }
    return output;
}

function decode2(base64) {
    const bytes = atob(base64).split('').map(c => c.charCodeAt(0));
    return new TextDecoder().decode(new Uint8Array(bytes));
}


/*

 */
import init, { encode3, decode3, encode4, decode4 } from './b64wasm/pkg/b64wasm.js';

async function loadWasmModule() {
    await init();
}

// BENCHMARK

function bench(encoder, decoder, iterations){
    // Warmup by running once
    for (const input of benchInputs) {
        const encoded = encoder(input);
        const decoded = decoder(encoded);
        if (input !== decoded) {
            console.log("Input: ", input);
            console.log("Encoded: ", encoded);
            console.log("Decoded: ", decoded);
            // Display decoded as hex for better debugging
            console.log("Decoded hex: ", Array.from(decoded).map(c => c.charCodeAt(0).toString(16)).join(' '));
            throw new Error('Invalid decoding');
        }
    }

    const nowHighRes = performance.now();
    for (let i = 0; i < iterations; i++) {
        for (const input of benchInputs) {
            const encoded = encoder(input);
            const decoded = decoder(encoded);
            if (input !== decoded) {
                console.log("Input: ", input);
                console.log("Encoded: ", encoded);
                console.log("Decoded: ", decoded);
                // Display decoded as hex for better debugging
                console.log("Decoded hex: ", Array.from(decoded).map(c => c.charCodeAt(0).toString(16)).join(' '));
                throw new Error('Invalid decoding');
            }
        }
    }
    return performance.now() - nowHighRes;
}

async function benchmark(){
    await loadWasmModule();

    // Disable button
    document.getElementById('run').disabled = true;

    // Clear table
    console.log('Base64 encoding and decoding benchmark');

    const encodersAndDecoders = [
        {name: 'Native', encode: encode1, decode: decode1},
        {name: 'TextEncoder', encode: encode2, decode: decode2},
        {name: 'Wasm (base64)', encode: encode3, decode: decode3},
        {name: 'Wasm (fast32)', encode: encode4, decode: decode4},
    ];


    let times = [];
    let speedIncreases = [];
    // Get iterations from iterations element
    const iterations = parseInt(document.getElementById('iterations').value);

    for (const {name, encode, decode} of encodersAndDecoders) {
        const time = bench(encode, decode,iterations);
        console.log(`${name} took ${time}ms`);
        times.push(time);
    }

    console.log('Benchmark finished');
    // Calculate speed increase over base (Native)
    const baseTime = times[0];
    for (let i = 0; i < times.length; i++) {
        const speedIncrease = baseTime / times[i];
        console.log(`${encodersAndDecoders[i].name} is ${speedIncrease.toFixed(2)}x faster than Native`);
        speedIncreases.push(speedIncrease.toFixed(2));
    }

    // Insert results into table
    const table = document.getElementById('benchmarkResults');
    // Clear table
    table.innerHTML = '';
    for (let i = 0; i < encodersAndDecoders.length; i++) {
        const row = table.insertRow(-1);
        const cell1 = row.insertCell(0);
        const cell2 = row.insertCell(1);
        const cell3 = row.insertCell(2);
        cell1.innerHTML = encodersAndDecoders[i].name;
        cell2.innerHTML = times[i];
        cell3.innerHTML = speedIncreases[i];
    }
    // Show table
    document.getElementById('benchmarkResults').style.display = 'table';
    // Enable button
    document.getElementById('run').disabled = false;
}

document.getElementById('run').addEventListener('click', benchmark);


async function process(){
    await loadWasmModule();
    // Get method from method select
    const method = document.getElementById('method').value;
    // Get input from input element
    const input = document.getElementById('input').value;

    let encoded, decoded;
    console.log('Method: ', method);
    const now = performance.now();
    switch (method) {
        case 'encode1':
            encoded = encode1(input);
            decoded = decode1(encoded);
            break;
        case 'encode2':
            encoded = encode2(input);
            decoded = decode2(encoded);
            break;
        case 'encode3':
            encoded = encode3(input);
            decoded = decode3(encoded);
            break;
        case 'encode4':
            encoded = encode4(input);
            decoded = decode4(encoded);
            break;
        default:
            console.error('Invalid method');
    }
    const elapsed = performance.now() - now;

    // Display results
    document.getElementById('encoded').innerText = encoded;
    document.getElementById('decoded').innerText = decoded;
    document.getElementById('time').value = elapsed.toString() + 'ms';
}

document.getElementById('process').addEventListener('click', process);