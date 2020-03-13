from re import compile

regex_r = [r'(\\\\)', r'(\\x..)', r'(\\")']
regex_c = list(map(compile, regex_r))


def analyze_one(string):
    for regex in regex_c:
        string = regex.sub('A', string)
    return len(string) - 2


def analyze_two(string):
    # wrong: 2894 is too high
    string = '"\\"' + string[1:-1].replace('\\', '\\\\').replace('\"', '\\\\\\"') + '\\""'
    return analyze_one(string), len(string)


if __name__ == '__main__':
    r_one, r_two, l_one, l_two = [0] * 4
    for line in open('input.txt'):
        l_one += len(line)
        r_one += analyze_one(line)
        data = analyze_two(line)
        r_two += data[0]
        l_two += data[1]
    print('result_one =', l_one - r_one)
    print('result_two =', l_two - r_two)
