import { configureStore } from "@reduxjs/toolkit";
import sessionReducer from "../store/sessionSlice";
import tasksSlice from "./tasksSlice";

export default configureStore({
    reducer: {
        session: sessionReducer,
        tasks: tasksSlice,
    },
});
