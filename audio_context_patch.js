const audioContextList = [];

function constructorPatch(obj, key) {
    obj[key] = new Proxy(obj[key], {
        construct(target, args) {
            const audioContext = new target(...args);
            audioContextList.push(audioContext);
            return audioContext;
        },
    });
}

constructorPatch(globalThis, 'AudioContext');

const userInputEventNames = [
    "mousedown",
    "pointerdown",
    "touchdown",
    "keydown",
];

function resumeAudioContexts() {
    let count = 0;
    audioContextList.forEach((context) => {
        if (context.state !== "running") {
            context.resume();
        } else {
            count++;
        }
    });
    if (count > 0 && count === audioContextList.length) {
        userInputEventNames.forEach((eventName) => {
            document.removeEventListener(eventName, resumeAudioContexts);
        });
    }
}

userInputEventNames.forEach((eventName) => {
    document.addEventListener(eventName, resumeAudioContexts);
});