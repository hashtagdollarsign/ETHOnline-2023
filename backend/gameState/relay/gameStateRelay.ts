
import WebSocket from 'ws';
import * as config from '../../config.json';


enum gameState {
    Up = "Up",
    Down = "Down",
    Left = "Left",
    Right = "Right",
    A = "A",
    B = "B",
    X = "X",
    Y = "Y",
}

const ws = new WebSocket(config.websocketServer)

let myState = gameState.X;
ws.send(myState);