// 2026년 05월 08일

// Santa is trying to deliver presents in a large apartment building,
// but he can't find the right floor - the directions he got are a little confusing.
// He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ),
// means he should go down one floor.
// The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

// For example:
// (()) and ()() both result in floor 0.
// ((( and (()(()( both result in floor 3.
// ))((((( also results in floor 3.
// ()) and ))( both result in floor -1 (the first basement level).
// ))) and )())()) both result in floor -3.
//
// To what floor do the instructions take Santa?

// part1. 산타가 가야할 층수를 구하라함
// ‘(’ = 층수+1
// ‘)’ = 층수-1

// 아놔 이거 변수명 뭐라고하지
const building: &str = "()(((()))(()()()((((()(((())(()(()((((((()(()(((())))((()(((()))((())(()((()()()()(((())(((((((())))()()(()(()(())(((((()()()((())(((((()()))))()(())(((())(())((((((())())))(()())))()))))()())()())((()()((()()()()(()((((((((()()())((()()(((((()(((())((())(()))()((((()((((((((())()((()())(())((()))())((((()())(((((((((((()()(((((()(()))())(((()(()))())((()(()())())())(()(((())(())())()()(()(()((()))((()))))((((()(((()))))((((()(()(()())())()(((()((((())((((()(((()()(())()()()())((()((((((()((()()))()((()))()(()()((())))(((()(((()))((()((()(()))(((()()(()(()()()))))()()(((()(((())())))))((()(((())()(()(())((()())))((((())))(()(()(()())()((()())))(((()((()(())()()((()((())(()()((())(())()))()))((()(())()))())(((((((()(()()(()(())())))))))(()((((((())((((())((())())(()()))))()(())(()())()())((())(()))))(()))(()((()))()(()((((((()()()()((((((((()(()(())((()()(()()))(())()())()((())))()))()())(((()))(())()(())()))()((()((()(()()())(())()()()((())())))((()()(()()((()(())()()())(((()(()()))))(())))(()(()())()))()()))))))()))))((((((())))())))(()(())())(()())))))(()))()))))))()((()))))()))))(()(()((()())())(()()))))(((())()))())())())(((()(()()))(())()(())(())((((((()()))))((()(()))))))(()))())(((()()(()))()())()()()())))))))))))))(())(()))(()))((()(())(()())(())())(()())(())()()(()())))()()()))(())())()))())())(()(())((())))))))(())))(())))))()))))((())(()(((()))))(()))()((()(())))(()())(((((()))()())()()))))()))))()))())(()(()()()))()))))))((()))))))))))()((()))((()(())((())()()(()()))()(()))))()()(()))()))(((())))(())()((())(())(()())()())())))))))())))()((())))()))(()))()()))(((((((()))())(()()))(()()(()))()(()((()())()))))))(((()()()())))(())()))()())(()()))()()))))))))(())))()))()()))))))()))()())))()(())(())))))()(())()()(()()))))())((()))))()))))(()(((((()))))))))())))())()(())()()))))(())))())()()())()()())()(()))))()))()))))))))())))((()))()))()))())))()())()()())))())))(()((())()((()))())))))())()(())((())))))))))))())()())(())())())(()))(()))()))())(()(())())()())()()(()))))(()(())))))))(())))())(())))))))())()()(())())())))(())))))()))()(()())()(()))())())))))()()(()))()))))())))))))))()))))()))))))())()())()()";

// 내가 파이썬으로 짰던것 rust로 직역
fn part1_00(a: &str) {
    let mut aa: i32 = 0; // 산타층수

    // 문자열의 길이를 출력 (바이트 단위)
    println!("{}", a.len());

    // 문자열을 순회하며 비교
    for i in a.chars() {
        if i == '(' {
            aa += 1;
        } else if i == ')' {
            aa -= 1;
        }
    }

    println!("{}", aa);
}

// ai가 러스트 그뭐더라 만든사이의 의도에 맞춰서 교정한 구조
fn part1_01(a: &str) {
    println!("{}", a.len());

    // fold를 사용하여 누적값을 계산합니다.
    let aa = a.chars().fold(0, |acc, c| {
        match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc, // 괄호가 아닌 다른 문자가 있을 경우 대비
        }
    });

    println!("{}", aa);
}

// Now, given the same instructions,
// find the position of the first character that causes him to enter the basement (floor -1).
// The first character in the instructions has position 1,
// the second character has position 2, and so on.

// For example:
// ) causes him to enter the basement at character position 1.
// ()()) causes him to enter the basement at character position 5.
//What is the position of the character that causes Santa to first enter the basement?

// 산타가 지하로 진입하는 지점 구하라함

// 파이썬으로 짠거 직역
fn part2_00(a: &str) {
    let mut aa = 0; // 파이썬의 aa (괄호 합계)
    let mut aaa = 0; // 파이썬의 aaa (인덱스 추적)

    // 파이썬: for i in a:
    for i in a.chars() {
        // 파이썬: aaa += 1
        aaa += 1;

        // 파이썬: if i == '(': aa += 1
        if i == '(' {
            aa += 1;
        }

        // 파이썬: if i == ')': aa -= 1
        if i == ')' {
            aa -= 1;
        }

        // 파이썬: if aa < 0: break
        if aa < 0 {
            break;
        }
    }

    // 파이썬: print(aa)
    println!("{}", aa);
    // 파이썬: print(aaa)
    println!("{}", aaa);
}

fn part2_01(a: &str) {
    let a = "()(((())) ... "; // 여기에 긴 문자열이 들어갑니다.

    let mut aa = 0; // 괄호 중첩 레벨
    let mut aaa = 0; // 처리된 문자 수 (인덱스 추적)

    // .chars()는 문자들을 가져오고, .enumerate()는 (인덱스, 문자) 쌍을 제공합니다.
    for (index, i) in a.chars().enumerate() {
        aaa = index + 1; // 현재 몇 번째 글자인지 저장 (1부터 시작하는 위치)

        if i == '(' {
            aa += 1;
        } else if i == ')' {
            aa -= 1;
        }

        // 닫는 괄호가 더 많아지는 순간 루프 중단
        if aa < 0 {
            break;
        }
    }

    println!("{}", aa);
    println!("{}", aaa);
}

fn part2_02(a: &str) {
    // 괄호 균형이 깨지는 첫 번째 위치를 찾는 방법
    let mut balance = 0;
    let position = a.chars().enumerate().find_map(|(i, c)| {
        if c == '(' {
            balance += 1;
        } else if c == ')' {
            balance -= 1;
        }

        if balance < 0 { Some(i + 1) } else { None }
    });

    println!("{:?}", position); // balance가 0 미만이 된 위치 출력
}

fn main() {
    part1_00(building);
    part1_01(building);
    part2_00(building);
    part2_01(building);
    part2_02(building);
}
