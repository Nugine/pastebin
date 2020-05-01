import "bootstrap/dist/css/bootstrap.min.css";
import "prismjs/themes/prism-coy.css";
import "font-awesome/css/font-awesome.min.css";
import "./index.css";

import React from "react";
import ReactDOM from "react-dom";
import App from "./app/App";

ReactDOM.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
    document.getElementById("root")
);
