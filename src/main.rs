use std::io;

fn main() {
    let point_a = read_point_from_input();
    let point_b = read_point_from_input();
    let point_c = read_point_from_input();
    let radius = read_line().trim().parse().expect("Not a number!");
    let result = find_thread_length(point_a, point_b, point_c, radius);
    println!("{:.2}", result);
}

fn find_thread_length(a: Point, b: Point, c: Point, r: f64) -> f64 {
    let line_ab = get_distance(&a, &b);
    let line_ac = get_distance(&a, &c);
    let line_bc = get_distance(&b, &c);

    let angle_c = acos((sqr(line_ac) + sqr(line_bc) - sqr(line_ab)) / (2.0 * line_ac * line_bc));
    let angle_a = acos(r / line_ac);
    let angle_b = acos(r / line_bc);

    if angle_a + angle_b >= angle_c {
        return line_ab;
    } else {
        return (angle_c - angle_a - angle_b) * r
            + sqrt(sqr(line_ac) - sqr(r))
            + sqrt(sqr(line_bc) - sqr(r));
    }
}

fn read_point_from_input() -> Point {
    let input = read_line();
    let mut args = input.split_whitespace();
    let point = Point {
        x: args.next().unwrap().parse().expect("Not a number!"),
        y: args.next().unwrap().parse().expect("Not a number!"),
        z: args.next().unwrap().parse().expect("Not a number!"),
    };
    return point;
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn acos(mut number: f64) -> f64 {
    if number > 1.0 {
        number = 1.0;
    }
    if number < -1.0 {
        number = -1.0;
    }
    return number.acos();
}

fn get_distance(start: &Point, end: &Point) -> f64 {
    return sqrt(sqr(start.x - end.x) + sqr(start.y - end.y) + sqr(start.z - end.z));
}

fn sqr(number: f64) -> f64 {
    return number * number;
}

fn sqrt(number: f64) -> f64 {
    return number.sqrt();
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}
