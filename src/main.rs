pub mod htmlparser;
pub mod dom;

fn main() {
    let input = "<!doctype html><HTML><hEad iD=\"head\" class=\"TESting\" ></heaD><body ft-expand><img src='img.png' />Some body\n-5.98<!-- comment! --></body></html>".to_owned();

    let dom = htmlparser::parse(input);

    println!("{}", dom);
}
