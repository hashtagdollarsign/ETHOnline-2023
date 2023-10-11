export const hashtagInput = {
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
        const buttonMapping = {
            ';': 0,  // A
            '\'': 1, // B
            'l': 2,  // X
            'p': 3,  // Y
            'q': 4, // LeftShoulder (LA)
            'e': 5, // RightShoulder (RA)
            'z': 6, // LeftTrigger (LB)
            'c': 7, // RightTrigger (RB)
            'Enter': 9, // Menu (Start)
            '.': 10, // LeftThumb
            '/': 11, // RightThumb
            't': 12, // dUp
            'g': 13, // dDown
            'f': 14, // dLeft
            'h': 15, // dRight
        }

        const axisMapping = {
            'a': [0, -1],
            'd': [0, 1],
            'w': [1, -1],
            's': [1, 1],
            'ArrowLeft': [2, -1],
            'ArrowRight': [2, 1],
            'ArrowUp': [3, -1],
            'ArrowDown': [3, 1]
        }

        window.addEventListener('keydown', function (e) {
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

        window.addEventListener('keyup', function (e) {
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
        navigator.getGamepads = function () {
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
        navigator.getGamepads = hashtagInput.getGamepads;
    },
    connect: function () {
        const event = new Event("gamepadconnected");
        hashtagInput.htdsController.connected = true;
        hashtagInput.htdsController.timestamp = Math.floor(Date.now() / 1000);
        event.gamepad = hashtagInput.htdsController;
        window.dispatchEvent(event);
    },
    disconnect: function () {
        const event = new Event("gamepaddisconnected");
        hashtagInput.htdsController.connected = false;
        hashtagInput.htdsController.timestamp = Math.floor(Date.now() / 1000);
        event.gamepad = hashtagInput.htdsController;
        window.dispatchEvent(event);
    },
};

window.hashtagInput = hashtagInput;
hashtagInput.create();
hashtagInput.connect();