import { createSlice } from "@reduxjs/toolkit";

export const tasksSlice = createSlice({
    name: "tasks",
    initialState: {
        list: [],
    },
    reducers: {
        setTasks: (state, action) => {
            state.list = action.payload;
        },
        clearTasks: (state) => {
            state.list = [];
        },
    },
});

export const { setTasks, clearTasks } = tasksSlice.actions;

export default tasksSlice.reducer;
