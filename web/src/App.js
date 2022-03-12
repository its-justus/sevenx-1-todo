import { React, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import axios from "axios";
import RegisterUser from "./components/RegisterUser/RegisterUser";

function App() {
    const { message, setMessage } = useState("");

    function fetchMessage() {
        axios.get("http://localhost:9001/hello").then((res) => {
            console.log(res);
        });
    }

    return (
        <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <button onClick={fetchMessage}>Ring Ring?</button>
                <RegisterUser />
            </header>
        </div>
    );
}

export default App;
