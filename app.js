import init, { encrypt_message } from './pkg/my_encryption_app.js';

async function main() {
    await init(); // Initialize the Wasm module

    document.getElementById('encrypt').addEventListener('click', () => {
        const publicKey = document.getElementById('public-key').value;
        const message = document.getElementById('message').value;

        try {
            const encrypted = encrypt_message(publicKey, message);
            document.getElementById('output').textContent = encrypted;
        } catch (error) {
            console.error(error);
            document.getElementById('output').textContent = "Error encrypting message.";
        }
    });
}

main();
