function objectPatch(obj, key, callbackBefore, callbackAfter) {
    obj[key] = new Proxy(obj[key], {
        construct(target, args) {
            if (typeof callbackBefore === 'function') {
                callbackBefore(target, args);
            }
            let result = new target(...args);
            if (typeof callbackAfter === 'function') {
                callbackAfter(result, args);
            }
            return result;
        },
        apply(target, thisArg, args) {
            if (typeof callbackBefore === 'function') {
                let stop = callbackBefore(thisArg, args);
                if (stop === true) {
                    return;
                }
            }
            let result = Reflect.apply(target, thisArg, args);
            if (typeof callbackAfter === 'function') {
                callbackAfter(thisArg, result, args);
            }
            return result;
        },
    });
}

let customKey = Symbol.for('custom-key');
const audioContextList = [];

objectPatch(globalThis, 'AudioContext', undefined, (thisArg) => {
    thisArg[customKey] = { state: '' };
    audioContextList.push(thisArg);
});
objectPatch(AudioContext.prototype, 'createBufferSource', undefined, (audioContext, thisArg) => {
    let descriptor = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Object.getPrototypeOf(thisArg)), 'onended');
    Object.defineProperty(thisArg, 'onended', {
        configurable: true,
        get: function () {
            return descriptor.get.call(this);
        },
        set: function (originalOnended) {
            return descriptor.set.call(this, () => {
                audioContext[customKey].state = 'ended';
                return originalOnended();
            });
        }
    })
});
objectPatch(AudioBufferSourceNode.prototype, 'stop', (thisArg) => {
    thisArg.context[customKey].state = 'ended';
});
objectPatch(AudioBufferSourceNode.prototype, 'start', (thisArg) => {
    if (thisArg.context[customKey].state !== 'started') {
        thisArg.context[customKey].state = 'started';
        return false;
    }
    return true; // skip play() call
});

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