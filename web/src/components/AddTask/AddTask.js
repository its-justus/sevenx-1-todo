import axios from "axios";
import React, { useState } from "react";
import { useSelector } from "react-redux";

function AddTask() {
    const [text, setText] = useState("");
    const personid = useSelector((state) => state.session.id);

    function addTask() {
        axios
            .post(
                "http://localhost:9001/task",
                { personid, text },
                { "Content-Type": "application/json" }
            )
            .then((res) => {})
            .catch((err) => {});
    }

    return (
        <form onSubmit={(e) => e.preventDefault()}>
            <input onChange={(e) => setText(e.target.value)} />
            <button onClick={addTask}>Add Task</button>
        </form>
    );
}

export default AddTask;
