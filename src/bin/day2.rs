// line format: "[min]-[max] [letter]: [password]"
fn split_line(line: &str) -> Option<(usize, usize, char, &str)> {
    let mut line_split = line.split(' ');
    let nums = line_split.next()?;
    let letter = line_split.next()?.chars().next()?;
    let password = line_split.next()?;
    let mut nums_split = nums.split('-');
    let min = nums_split.next()?.parse().ok()?;
    let max = nums_split.next()?.parse().ok()?;
    Some((min, max, letter, password))
}

fn main() {
    let input = include_str!("day2.txt");
    let lines: Vec<&str> = input.lines().collect();

    // PART 1)
    //  Each line gives the password policy and then the password.
    //  The password policy indicates the lowest and highest number
    //  of times a given letter must appear for the password to be
    //  valid. For example, 1-3 a means that the password must
    //  contain a at least 1 time and at most 3 times.
    let valid_passwords: usize = lines
        .iter()
        .filter(|line| {
            let (min, max, letter, password) = split_line(line).unwrap();
            let letter_count = password.chars().filter(|c| c == &letter).count();
            letter_count >= min && letter_count <= max
        })
        .count();
    println!("{}", valid_passwords);

    // PART 2)
    //  Each policy actually describes two positions in the password,
    //  where 1 means the first character, 2 means the second character,
    //  and so on. (Be careful; Toboggan Corporate Policies have no
    //  concept of "index zero"!) Exactly one of these positions must
    //  contain the given letter. Other occurrences of the letter are
    //  irrelevant for the purposes of policy enforcement.
    let valid_passwords: usize = lines
        .iter()
        .filter(|line| {
            let (a, b, letter, password) = split_line(line).unwrap();
            let chars: Vec<char> = password.chars().collect();
            let (a, b) = (chars[a - 1], (chars[b - 1]));
            (a == letter || b == letter) && (a == letter) != (b == letter)
        })
        .count();
    println!("{}", valid_passwords);
}