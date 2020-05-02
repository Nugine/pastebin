import "./App.css";

import React from "react";
import Editor from "./editor/Editor";
import { BrowserRouter as Router, Switch, Route, Redirect } from "react-router-dom";
import Show from "./show/Show";
import Footer from "./Footer";

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
