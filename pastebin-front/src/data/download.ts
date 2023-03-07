export function isValidFileName(s: string): boolean {
    if (s === "") return false;
    const invalidChars = "~`!@#$%^&*()-+={}[]|:;\"'<>,.?/\b\f\n\r\t\v\\\0";
    for (const ch of s.split("")) {
        if (invalidChars.indexOf(ch) !== -1) return false;
    }
    return true;
}

export function downloadFile(filename: string, content: string) {
    const a = document.createElement("a");
    a.download = filename;
    a.href = URL.createObjectURL(new Blob([content]));
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
}
