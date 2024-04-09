function isPalindrome(x: number): boolean {
	return (x.toString().split("").reverse().join("")) === x.toString();
}

const tests = [121, -121];
tests.forEach((n) => console.log(`${n} -> ${isPalindrome(n)}`));
