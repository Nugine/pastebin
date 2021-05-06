import "./App.css";

import React, { useEffect } from "react";
import { BrowserRouter as Router, Switch, Route, Redirect } from "react-router-dom";
import { observer, useLocalObservable } from "mobx-react-lite";
import Container from "react-bootstrap/Container";

import Editor from "./pages/Editor";
import Show from "./pages/Show";

function useDelay(f: () => void, ms: number): void {
    useEffect(() => {
        const timer = setTimeout(f, ms);
        return () => clearTimeout(timer);
    });
}

const Footer: React.FC = observer(() => {
    const v = useLocalObservable(() => ({
        displayRepo: false
    }));

    useDelay(() => v.displayRepo = true, 1000);

    const nowYear = new Date().getFullYear();

    const authorLink = (
        <a href="https://github.com/Nugine" target="_blank" rel="noopener noreferrer">
            Nugine
        </a>
    );

    return (
        <footer style={{ marginBottom: "1em", textAlign: "center" }}>
            <span>
                © {nowYear > 2019 ? `2019 - ${nowYear}` : "2019"} {authorLink}
            </span>
            {v.displayRepo ? (
                <span style={{ marginLeft: "0.5em" }}>
                    <a href="https://github.com/Nugine/pastebin" target="_blank" rel="noopener noreferrer">
                        <img
                            alt="GitHub stars"
                            src="https://img.shields.io/github/stars/Nugine/pastebin?style=social"
                        />
                    </a>
                </span>
            ) : null}
        </footer>
    );
});

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

    return (
        <Container fluid="lg" style={{
            minHeight: "100vh",
            display: "flex",
            flexDirection: "column",
            alignItems: "center"
        }}>
            {router}
            <Footer />
        </Container>
    );

};

export default App;
