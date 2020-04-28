import React from "react";

const defaultEditor: { isValid: boolean | null, updateIsValid: (val: boolean) => void } = {
    isValid: null,
    updateIsValid: () => { }
};

export const EditorContext = React.createContext(defaultEditor);
