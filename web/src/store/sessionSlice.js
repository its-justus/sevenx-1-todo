import { createSlice } from "@reduxjs/toolkit";

export const sessionSlice = createSlice({
    name: "session",
    initialState: {
        id: undefined,
        login: undefined,
    },
    reducers: {
        setSession: (state, action) => {
            state.id = action.payload.id;
            state.login = action.payload.login;
        },
        resetSession: (state) => {
            state.id = undefined;
            state.login = undefined;
        },
        setId: (state, action) => {
            state.id = action.payload;
        },
        resetId: (state) => {
            state.id = "";
        },
    },
});

export const { setId, resetId, setSession, resetSession } =
    sessionSlice.actions;

export default sessionSlice.reducer;
