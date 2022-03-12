import React from "react";
import { useSelector } from "react-redux";

function User() {
    const login = useSelector((state) => state.session.login);
    return (
        <div>
            <span>{login ? "Hiya, " + login : ""}</span>
        </div>
    );
}

export default User;
