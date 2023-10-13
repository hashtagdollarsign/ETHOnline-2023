import config from '../../config.json';
const { websocketServer } = config;
enum GameState {
    Up = "Up",
    Down = "Down",
    Left = "Left",
    Right = "Right",
    A = "A",
    B = "B",
    X = "X",
    Y = "Y",
}

const ws = new WebSocket(websocketServer);

ws.addEventListener("open", (event) => {
    console.log("Connected to Server.");
});

ws.addEventListener("message", (event) => {
    console.log(event);
});

const sendEvent = (event: GameState) => {
    const sendData = JSON.stringify({
        action: "game",
        game_move: event
    })

    ws.send(sendData);
    console.log("sent.");
}



export {GameState, sendEvent}