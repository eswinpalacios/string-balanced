function isStringBalanced(characters) {
	open = { '{': null, '[': null, '(': null }
	close = { '}': '{', ']': '[', ')': '(' }
	stack = []

	for (let i in characters) {
		const character = characters[i]
		if (open[character] !== undefined)
			stack.push(character)
		else if (close[character] !== undefined) {
			if (stack.length == 0)
				return false
			else if (stack[stack.length - 1] != close[character])
				return false
			else
				stack.pop()
		}
	}

	return stack.length == 0 ? true : false;
}

console.log(isStringBalanced('(a[0]+b[2c[6]]) {24+53})')); // false
console.log(isStringBalanced('f(e(d))[()]{}([])')); // true
console.log(isStringBalanced('((b)')); // false
console.log(isStringBalanced('(c]')); // false
console.log(isStringBalanced('{(a[])')); // false
console.log(isStringBalanced('([)]')); // false
console.log(isStringBalanced(')(')); // false
console.log(isStringBalanced('')); // true
console.log(isStringBalanced('([)]')); // false
