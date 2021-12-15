fn increment(s: &mut [char]) {
    for i in 0..s.len() {
        let v = &mut s[s.len() - 1 - i];
        if *v == 'z' {
            *v = 'a';
        } else {
            *v = ((*v as u8) + 1) as char;
            break;
        }
    }
}

fn has_straight(s: &[char], size: usize) -> bool {
    s.windows(size)
        .any(|cs| cs[1] as u8 == (cs[0] as u8) + 1 && cs[2] as u8 == (cs[1] as u8) + 1)
}

fn has_n_doubles(s: &[char], n: i32) -> bool {
    let mut found = 0;
    let mut skip = false;
    for i in 0..(s.len() - 1) {
        if skip {
            skip = false;
        } else {
            if s[i] == s[i + 1] {
                found += 1;
                skip = true;
            }
        }
    }
    found >= n
}

fn find_next_password(s: &mut [char]) {
    loop {
        if has_n_doubles(s, 2)
            && has_straight(s, 3)
            && !s.iter().any(|&c| c == 'i' || c == 'o' || c == 'l')
        {
            return;
        }
        increment(s);
    }
}

fn main() {
    let mut s = aoc::get_input(15, 11).trim().chars().collect::<Vec<_>>();
    find_next_password(&mut s);
    println!("Part 1: {}", s.iter().collect::<String>());
    increment(&mut s);
    find_next_password(&mut s);
    println!("Part 2: {}", s.iter().collect::<String>());
}
