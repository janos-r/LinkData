const threshold = 2;

export const findOverlap = (
    minLength: number,
    a: number[],
    b: number[],
): number => {
    const maxLength = Math.min(a.length, b.length);
    // the revers iteration causes a non-greedy pattern search, matching as much as possible
    // but slower on long lists with short matches
    for (let i = maxLength; i >= minLength; i--) {
        if (JSON.stringify(a.slice(i * -1)) === JSON.stringify(b.slice(0, i))) {
            return i;
        }
    }
    return 0;
};

export const merge = (a: number[], b: number[]): number[] => {
    const len = findOverlap(threshold, a, b);
    return a.concat(b.slice(len));
};
