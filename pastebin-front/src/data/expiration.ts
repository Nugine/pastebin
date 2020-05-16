import { Expiration } from "./index";

const expirationArray: Expiration[] = [];


expirationArray.push({
    value: 3600,
    display: "one hour",
});

expirationArray.push({
    value: 3600 * 24,
    display: "one day",
});

expirationArray.push({
    value: 3600 * 24 * 3,
    display: "three days",
});


expirationArray.push({
    value: 3600 * 24 * 7,
    display: "one week",
});


expirationArray.push({
    value: 3600 * 24 * 30,
    display: "one month",
});


export function getExpirationArray(): Expiration[] {
    return expirationArray;
}
