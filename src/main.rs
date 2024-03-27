fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
}

fn main1() {
    let a = String::from("abc");
    let b = a.clone();
    let c = b.clone();
    println!("{a}");
    println!("{b}");
    println!("{c}");
}

fn main2() {
    let a = 20240327;
    let b = a;
    let c = b;
    println!("{a}");
    println!("{b}");
    println!("{c}");
}

fn main3() {
    let mut a = 10u32;
    let main3_r1 = &mut a;
    let main3_r2 = main3_r1;
    //println!("{main3_r1}");
    println!("{main3_r2}");
}

fn main4() {
    let a = 10u32;
    let main4_r1 = a;
    let main4_r2 = main4_r1;
    println!("{main4_r1}");
    println!("{main4_r2}");
}

fn main5() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    //let d = &mut a;
    println!("{b}");
}    