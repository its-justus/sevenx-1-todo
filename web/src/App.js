import React from "react";
import logo from "./logo.svg";
import "./App.css";
import axios from "axios";
import RegisterUser from "./components/RegisterUser/RegisterUser";
import Login from "./components/Login/Login";
import User from "./components/User/User";
import AddTask from "./components/AddTask/AddTask";
import Tasks from "./components/Tasks/Tasks";

function App() {
    function fetchMessage() {
        axios.get("http://localhost:9001/hello").then((res) => {
            console.log(res);
        });
    }

    return (
        <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <User />
                <button onClick={fetchMessage}>Ring Ring?</button>
                <RegisterUser />
                <Login />
                <AddTask />
                <Tasks />
            </header>
        </div>
    );
}

export default App;
