export interface Expiration {
    value: number;
    display: string;
}

export const EXPIRATIONS: Expiration[] = [
    {
        value: 3600,
        display: "1 小时",
    },
    {
        value: 3600 * 24,
        display: "1 天",
    },
    {
        value: 3600 * 24 * 3,
        display: "3 天",
    },
    {
        value: 3600 * 24 * 7,
        display: "7 天",
    },
    {
        value: 3600 * 24 * 30,
        display: "30 天",
    },
];

export const DEFAULT_EXPIRATION = EXPIRATIONS[2].value;
