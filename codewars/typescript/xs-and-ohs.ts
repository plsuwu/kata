export function xo(str: string): boolean {
    let xcounter = 0;
    let ocounter = 0;

    str.toLowerCase().split("").forEach((char) => {
        if (char === "x" || char === "o") {
            char === "x" ? ++xcounter : ++ocounter;
        };
    });

    return xcounter === ocounter;
}

console.log(xo('xo'));
console.log(xo('ooox'));
console.log(xo('asdf'));
