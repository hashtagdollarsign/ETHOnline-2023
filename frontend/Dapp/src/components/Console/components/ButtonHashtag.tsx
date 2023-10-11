import React, { useState } from 'react';
import { Button } from '@mui/material';
import { hashtagInput, ButtonMapping, Controller, HashtagInput } from './hashtagInputHandler';

const ButtonHashtag: React.FC = () => {
    const [isConnected, setIsConnected] = useState<boolean>(hashtagInput.htdsController.connected);

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
    };

    const handleButtonDown = (index: number): void => {
        if (hashtagInput.htdsController.buttons[index].value === 0) {
            hashtagInput.htdsController.timestamp = Math.floor(Date.now());
        }
        hashtagInput.htdsController.buttons[index].value = 1;
    }

    const handleButtonUp = (index: number): void => {
        hashtagInput.htdsController.buttons[index].value = 0;
        hashtagInput.htdsController.timestamp = Math.floor(Date.now());
    }

    const toggleConnection = (): void => {
        if (isConnected) {
            hashtagInput.disconnect();
        } else {
            hashtagInput.connect();
        }
        setIsConnected(!isConnected);
    }

    return (
        <div>
            {Object.entries(buttonMapping).map(([key, index]) => (
                <Button 
                    key={key}
                    variant="contained"
                    onMouseDown={() => handleButtonDown(index)}
                    onMouseUp={() => handleButtonUp(index)}
                >
                    {key}
                </Button>
            ))}
             <Button 
                variant="contained" 
                color="primary" 
                onClick={toggleConnection}
            >
                {isConnected ? 'Disconnect' : 'Connect'}
            </Button>
        </div>
    );
}

export default ButtonHashtag;
