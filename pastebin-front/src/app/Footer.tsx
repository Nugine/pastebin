import React, { useState, useEffect } from "react";

const Footer: React.FC = () => {
    const nowYear = new Date().getFullYear();

    const authorLink = (
        <a href="https://github.com/Nugine" target="_blank" rel="noopener noreferrer">
            Nugine
        </a>
    );

    const copyrightSpan = (
        <span>© {nowYear > 2019 ? `2019 - ${nowYear}` : "2019"} {authorLink}. </span>
    );

    const [delayFlag, setDelayFlag] = useState(true);

    useEffect(() => {
        setTimeout(() => setDelayFlag(false), 1000);
    });

    const repoSpan = delayFlag ? null : (
        <span>
            <a href="https://github.com/Nugine/pastebin" target="_blank" rel="noopener noreferrer">
                <img
                    alt="GitHub stars"
                    src="https://img.shields.io/github/stars/Nugine/pastebin?style=social"
                />
            </a>
        </span>
    );

    return (
        <footer style={{ marginBottom: "1rem", textAlign: "center" }}>
            {copyrightSpan}
            {repoSpan}
        </footer>
    );
};

export default Footer;
