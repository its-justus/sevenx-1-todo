import axios from "axios";
import { React, useState } from "react";

function RegisterUser() {
    const [name, setName] = useState("");

    function register() {
        axios.post(
            "http://localhost:9001/register",
            { login: name },
            {
                headers: { "Content-Type": "application/json" },
            }
        );
    }

    return (
        <form onSubmit={(e) => e.preventDefault()}>
            <label>Username</label>
            <input
                type={"text"}
                onChange={(e) => setName(e.target.value)}
            ></input>
            <button onClick={register}>Register</button>
        </form>
    );
}

export default RegisterUser;
