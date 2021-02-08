def is_string_balanced(characters):
    open = {'{': None, '[': None, '(': None}
    close = {'}': '{', ']': '[', ')': '('}
    stack = []

    for character in characters:
        if character in open:
            stack.append(character)
        elif character in close:
            if len(stack) == 0:
                return False
            elif stack[len(stack) - 1] != close[character]:
                return False
            else:
                stack.pop()

    return len(stack) == 0


print(is_string_balanced('(a[0]+b[2c[6]]) {24+53})'))  # true
print(is_string_balanced('f(e(d))[()]{}([])'))  # true
print(is_string_balanced('((b)'))  # false
print(is_string_balanced('(c]'))  # false
print(is_string_balanced('{(a[])'))  # false
print(is_string_balanced('([)]'))  # false
print(is_string_balanced(')('))  # false
print(is_string_balanced(''))  # true
print(is_string_balanced('([)]'))  # false
