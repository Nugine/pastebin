import { useEffect } from "react";

export function delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(() => resolve(), ms));
}

export function useDelay(f: () => void, ms: number): void {
    useEffect(() => {
        const timer = setTimeout(f, ms);
        return () => clearTimeout(timer);
    });
}