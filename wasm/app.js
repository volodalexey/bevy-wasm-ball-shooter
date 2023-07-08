import init from './wasm_bindgen_core.js'

async function run() {
    await init().catch(err => {
        if (!err.message.startsWith("Using exceptions for control flow,")) {
            throw err;
        }
    });

    const loader = document.querySelector('.loader')
    if (loader != null) {
        loader.parentElement?.removeChild(loader)
    }

    // disable touch context menu
    document.addEventListener('contextmenu', (e) => {
        if (e.pointerType === "touch") {
            e.preventDefault();
        }
    });
}

run().catch((err) => {
    const loader = document.querySelector('.loader')
    if (loader != null) {
        loader.parentElement?.removeChild(loader)
    }
    console.error(err)
    const errorMessageDiv = document.querySelector('.error-message')
    if (errorMessageDiv != null) {
        errorMessageDiv.classList.remove('hidden')
        errorMessageDiv.innerText = ((Boolean(err)) && (Boolean(err.message))) ? err.message : err
    }
    const errorStackDiv = document.querySelector('.error-stack')
    if (errorStackDiv != null) {
        errorStackDiv.classList.remove('hidden')
        errorStackDiv.innerText = ((Boolean(err)) && (Boolean(err.stack))) ? err.stack : ''
    }
    const canvas = document.querySelector('canvas')
    if (canvas != null) {
        canvas.parentElement?.removeChild(canvas)
    }
})
