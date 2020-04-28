import "bootstrap/dist/css/bootstrap.min.css";
import "./App.css";

import React from "react";
import Editor from "./editor/Editor";
import { BrowserRouter as Router, Switch, Route, Redirect } from "react-router-dom";
import Show from "./show/Show";

import Container from "react-bootstrap/Container";
import Col from "react-bootstrap/Col";


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
        <Container className="full-screen">
            <Col xs={12} sm={12} md={10} className="full-screen">
                <div style={{ minHeight: "calc(100vh - 7rem)", width: "100%" }}>
                    {router}
                </div>
            </Col>
        </Container>
    );
};

export default App;
