export function countBits(n: number): number {
    console.log((n >>> 0).toString(2));

    return n
        .toString(2)
        .split("")
        .filter((e) => e === "1")
        .length;
}

console.log(countBits(256));
console.log(countBits(1234));

console.log(countBits(6628025774274465));
