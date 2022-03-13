import axios from "axios";
import React, { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { setTasks, clearTasks } from "../../store/tasksSlice";

function Tasks() {
    const tasks = useSelector((state) => state.tasks.list);
    const userid = useSelector((state) => state.session.id);
    const dispatch = useDispatch();

    function fetchTasks() {
        const params = new URLSearchParams();
        params.append("id", String(userid));
        axios
            .get("http://localhost:9001/tasks", {
                params: params,
                "Content-Type": "application/x-www-form-urlencoded",
            })
            .then((res) => {
                console.log(res);
                dispatch(setTasks(res.data.tasks));
            })
            .catch((err) => {
                console.log(err);
            });
    }
    console.log("rendering Tasks");

    return (
        <ul>
            <li key="button">
                <button onClick={fetchTasks}>Get Tasks</button>
            </li>
            {tasks.map((task, idx) => {
                return <li key={task.id}>{task.text}</li>;
            })}
        </ul>
    );
}

export default Tasks;
