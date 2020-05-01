import "./App.css";

import React from "react";
import Editor from "./editor/Editor";
import { BrowserRouter as Router, Switch, Route, Redirect } from "react-router-dom";
import Show from "./show/Show";

import Container from "react-bootstrap/Container";

const App: React.FC = () => {
    const router = (
        <Router>
            <Switch>
                <Route exact strict path="/:key/" component={Show} />
                <Route exact strict path="/" component={Editor} />
                <Redirect exact strict from="/:key" to="/:key/" />
                <Redirect to="/" />
            </Switch>
        </Router>
    );

    const nowYear = new Date().getFullYear();

    const authorLink = (
        <a href="https://github.com/Nugine" target="_blank" rel="noopener noreferrer">
            Nugine
        </a>
    );

    const repoLink = (
        <a href="        https://github.com/Nugine/pastebin
        " target="_blank" rel="noopener noreferrer">
            <img
                alt="GitHub stars"
                src="https://img.shields.io/github/stars/Nugine/pastebin?style=social"
            />
        </a>
    );

    const footer = (
        <footer style={{ marginBottom: "1rem", textAlign: "center" }}>
            <span>© {nowYear > 2019 ? `2019 - ${nowYear}` : "2019"} {authorLink}. </span>
            <span>{repoLink}</span>
        </footer>
    );

    return (
        <Container fluid="lg" style={{
            minHeight: "100vh",
            display: "flex",
            flexDirection: "column",
            alignItems: "center"
        }}>
            {router}
            {footer}
        </Container>
    );

};

export default App;
