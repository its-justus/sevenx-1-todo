import { configureStore } from "@reduxjs/toolkit";
import sessionReducer from "../store/sessionSlice";

export default configureStore({
    reducer: {
        session: sessionReducer,
    },
});
