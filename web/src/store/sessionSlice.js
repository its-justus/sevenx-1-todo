import { createSlice } from "@reduxjs/toolkit";

export const sessionSlice = createSlice({
    name: "session",
    initialState: {
        id: "",
    },
    reducers: {
        setId: (state, action) => {
            state.id = action.payload;
        },
        resetId: (state) => {
            state.id = "";
        },
    },
});

export const { setId, resetId } = sessionSlice.actions;

export default sessionSlice.reducer;
