pub mod htmlparser;
pub mod dom;

fn main() {
    let input = "<html><head> </head><body>Some body\n-5.98</body></html>".to_owned();

    let dom = htmlparser::parse(input);

    println!("{:?}", dom);
}
