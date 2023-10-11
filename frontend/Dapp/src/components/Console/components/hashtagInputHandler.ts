declare global {
    interface Event {
        gamepad?: Controller;
    }
    interface Window {
        hashtagInput: HashtagInput;
    }
}


export interface Button {
    pressed: boolean;
    touched: boolean;
    value: number;
}

export interface Controller {
    axes: number[];
    buttons: Button[];
    connected: boolean;
    id: string;
    index: number;
    mapping: string;
    timestamp: number;
}

export interface ButtonMapping {
    [key: string]: number;
}

export interface AxisMapping {
    [key: string]: number[];
}
export interface HashtagInput {
    getGamepads: (() => any) | null;
    htdsController: Controller;
    create: () => void;
    destroy: () => void;
    connect: () => void;
    disconnect: () => void;
}


export const hashtagInput: HashtagInput = {
    getGamepads: null,
    htdsController: {
        axes: [0, 0, 0, 0],
        buttons: [
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
            {
                pressed: false,
                touched: false,
                value: 0,
            },
        ],
        connected: false,
        id: "hashtagdollarsign Controller",
        index: 0,
        mapping: "standard",
        timestamp: Math.floor(Date.now() / 1000),
    },
    create: function () {
        const buttonMapping: ButtonMapping = {
            'A': 0,
            'B': 1,
            'X': 2,
            'Y': 3,
            'START': 9,
            'dUp': 12,
            'dDown': 13,
            'dLeft': 14,
            'dRight': 15
        }

        const axisMapping: AxisMapping = {
            'a': [0, -1],
            'd': [0, 1],
            'w': [1, -1],
            's': [1, 1],
            'ArrowLeft': [2, -1],
            'ArrowRight': [2, 1],
            'ArrowUp': [3, -1],
            'ArrowDown': [3, 1]
        }

        window.addEventListener('keydown', function (e: KeyboardEvent) {
            const buttonIndex = buttonMapping[e.key];
            if (buttonIndex !== undefined) {
                if (hashtagInput.htdsController.buttons[buttonIndex].value === 0) {
                    hashtagInput.htdsController.timestamp = Math.floor(Date.now());
                }
                hashtagInput.htdsController.buttons[buttonIndex].value = 1;
            }

            const axisInput = axisMapping[e.key];
            if (axisInput !== undefined) {
                if (hashtagInput.htdsController.axes[axisInput[0]] === 0) {
                    hashtagInput.htdsController.timestamp = Math.floor(Date.now());
                }
                hashtagInput.htdsController.axes[axisInput[0]] = axisInput[1];
            }
        })

        window.addEventListener('keyup', function (e: KeyboardEvent) {
            const buttonIndex = buttonMapping[e.key];
            if (buttonIndex !== undefined) {
                hashtagInput.htdsController.buttons[buttonIndex].value = 0;
                hashtagInput.htdsController.timestamp = Math.floor(Date.now());
            }

            const axisInput = axisMapping[e.key];
            if (axisInput !== undefined) {
                hashtagInput.htdsController.axes[axisInput[0]] = 0;
                hashtagInput.htdsController.timestamp = Math.floor(Date.now());
            }
        })

        hashtagInput.getGamepads = navigator.getGamepads;
        hashtagInput.getGamepads = function () {
            return [
                hashtagInput.htdsController,
                null,
                null,
                null
            ];
        };
    },

    destroy: function () {
        if (hashtagInput.htdsController.connected) {
            hashtagInput.disconnect();
        }
        hashtagInput.getGamepads = navigator.getGamepads.bind(navigator);
    },

    connect: function () {
        const event = new CustomEvent("gamepadconnected", {
            detail: {
                gamepad: hashtagInput.htdsController
            }
        });
        window.dispatchEvent(event);
    },

    disconnect: function () {
        const event = new CustomEvent("gamepaddisconnected", {
            detail: {
                gamepad: hashtagInput.htdsController
            }
        });
        window.dispatchEvent(event);
    },
};

window.hashtagInput = hashtagInput;
hashtagInput.create();
hashtagInput.connect();