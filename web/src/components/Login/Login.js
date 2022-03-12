import axios from "axios";
import React, { useState } from "react";
import { useDispatch } from "react-redux";
import { resetId, setId } from "../../store/sessionSlice";

function Login() {
    const [username, setUsername] = useState("");
    const dispatch = useDispatch();

    function submit() {
        console.debug("submitting login");
        axios
            .post(
                "http://localhost:9001/login",
                { login: username },
                { headers: { "Content-Type": "application/json" } }
            )
            .then((res) => {
                console.debug(res);
                dispatch(setId(res.data.id));
            })
            .catch((err) => {
                console.debug(err);
                dispatch(resetId());
            });
    }

    return (
        <form onSubmit={(e) => e.preventDefault()}>
            <input
                type={"text"}
                onChange={(e) => setUsername(e.target.value)}
            ></input>
            <button onClick={submit}>Login</button>
        </form>
    );
}

export default Login;
