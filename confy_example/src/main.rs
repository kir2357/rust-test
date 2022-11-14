mod initialize;

fn main() {
    println!("{:?}",initialize::CONFIG);
    println!("{:?}",initialize::CONFIG);
    println!("{:?}",*initialize::CONFIG);
    println!("{:?}",*initialize::CONFIG);
    println!("{:?}",initialize::CONFIG.address);
    println!("{:?}",initialize::CONFIG.address);

    let b = initialize::CONFIG.port;
    // let c = initialize::CONFIG.address;
    // copyトレイトがないと借用できないというエラー
    // 基本的に借用できないので参照する

    let c = &initialize::CONFIG.address;

    println!("{:?}",b);
    println!("{:?}",initialize::CONFIG.port);
    println!("{:?}",c);
}