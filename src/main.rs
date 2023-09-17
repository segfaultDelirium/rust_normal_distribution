use rand;
use rand::distributions::Uniform;
use rand_distr::{Normal, Distribution};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let die  = Uniform::from(1..8);

    for _i in 0..20{
        let x = die.sample(&mut rng);
        println!("random number = {x}");
        if x == 6{
            break;
        }
    }

    normal_distribution();
}


fn normal_distribution(){
    let mut rng = rand::thread_rng();

    let normal = Normal::new(1.0, 2.0).unwrap();
    let x = normal.sample(&mut rng);

    for _i in 0..20{
        let x = normal.sample(&mut rng);
        println!("random number = {x}");
    }
    // let normal_distribution = Norma

}