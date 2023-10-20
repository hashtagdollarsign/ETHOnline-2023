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


class GameStateServer {
    public ws: WebSocket;
    constructor() {
        this.ws = new WebSocket(websocketServer);

        this.ws.addEventListener("open", (event) => {
            console.log("Connected to Server.");
        });

        this.ws.addEventListener("message", (event) => {
            console.log(event);
        });
    }

    public send_event = (event: GameState) => {
        const sendData = JSON.stringify({
            action: "game",
            game_move: event
        })

        this.ws.send(sendData);
        console.log("sent.");
    }

    public hash = () => {
        const sendData = JSON.stringify({
            action: "hash",
            data: "Not implemented yet"
        })

        this.ws.send(sendData);
    }

    public de_hash = () => {
        const binary_data = [120, 156, 99, 98, 64, 128, 61, 75, 180, 82, 65, 180, 4, 155, 142, 0, 11, 144, 254, 178, 20, 194, 127, 81, 100, 32, 0, 0, 88, 77, 6, 18];

        // Convert the Uint8Array to a binary string
        const binary_string = String.fromCharCode.apply(null, binary_data);

        // Encode the binary string as base64
        const encoded_binary_string = btoa(binary_string);

        console.log(encoded_binary_string);

        let hashed_data = JSON.stringify({
            action: "de_hash",
            hashed_data: encoded_binary_string.toString()
        });

        console.log(hashed_data);

        this.ws.send(hashed_data);
        console.log("de_hash.");
    }

}



export {GameStateServer, GameState}