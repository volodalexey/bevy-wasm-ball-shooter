function createLogNode(messages) {
    let node = document.createElement("div");
    node.classList.add('log-node');
    for (let message of messages) {
        let textNode = document.createTextNode(message);
        node.appendChild(textNode);
    }
    return node;
}

let patched = [];
let consoleOutput = document.querySelector('.console-output');

function createLogger(originalConsoleMethod, logLevel) {
    return (...args) => {
        let logNode = createLogNode(args);
        logNode.classList.add(logLevel);
        consoleOutput.appendChild(logNode);
        originalConsoleMethod.apply(console, args);
    };
}

function patchConsole(method) {
    let originalConsoleMethod = console[method];
    console[method] = createLogger(originalConsoleMethod, method);
    patched.push({ method, originalConsoleMethod });
}

let collapseButton = document.querySelector('.collapse-button');

let persistantLog = localStorage.getItem('persistant-log');
let logEnabled = persistantLog === 'true';
if (logEnabled) {
    window.onerror = createLogger(console.error, 'error');
    patchConsole('log');
    patchConsole('warn');
    patchConsole('error');

    consoleOutput.classList.remove('hidden');
    collapseButton.classList.remove('hidden');
    collapseButton.addEventListener('click', () => {
        consoleOutput.classList.toggle('hidden');
    });
}

let consoleButton = document.querySelector('.console-button');
/*
function restorePatches() {
    for (let { method, originalConsoleMethod } of patched) {
        console[method] = originalConsoleMethod;
    }
    consoleOutput.parentElement?.removeChild(rootNode);
    delete window.onerror;
}
*/

consoleButton.addEventListener('click', () => {
    if (persistantLog === 'true') {
        localStorage.removeItem('persistant-log');
    } else {
        localStorage.setItem('persistant-log', 'true');
    }
    consoleButton.parentElement?.removeChild(consoleButton);
})

function checkOrLog() {
    setTimeout(() => {
        consoleButton.parentElement?.removeChild(consoleButton);
        if (!logEnabled) {
            collapseButton.parentElement?.removeChild(collapseButton);
            consoleOutput.parentElement?.removeChild(consoleOutput);
        }
    }, 3000);
}

checkOrLog();