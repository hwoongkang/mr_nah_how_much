use std::io::{self, Write};
fn main() {
    let bias_height = user_input_int("최애의 키를 적어보세용 뻥튀기 금지~".to_string());
    let user_height = user_input_int("님의 키를 적어보세용 ".to_string());
    let point_height = user_input_int("님 최애의 포인트 거리를 적어보세용  ".to_string());
    let ans = solve(bias_height, user_height, point_height);
    match ans {
        Ok(val) => println!("{}", val),
        Err(_) => println!("최애의 키가 님의 키보다 작아요"),
    }
    println!(
        "최애와 사진을 찍기 위해서 최애는 다리를 무려 이만큼 벌려야하네여...웨딩사진 찍을 때 참고!"
    )
}

fn solve(bias: f32, user: f32, point: f32) -> Result<u32, ()> {
    let i = bias - user;
    if i < 0.0 {
        return Err(());
    }
    let result = 2.0 * (2.0 * point * i - i * i).sqrt();

    Ok(result.round() as u32)
}

fn user_input_int(message: String) -> f32 {
    loop {
        let mut input = String::new();
        print!("{}", message);
        io::stdout().flush().unwrap();
        let Ok(_) = io::stdin().read_line(&mut input) else {
            println!("입력을 확인해주세여");
            continue;
        };
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("숫자로 입력해주세용");
                continue;
            }
        };
    }
}
