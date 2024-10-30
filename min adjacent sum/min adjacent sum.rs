use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.gen_range(10..100)); 
    }
    vec
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, (i32, i32))> {
    if data.len() < 2 {
        return None; 
    }

    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }

    Some((min_sum, min_pair))
}

fn print_min_adjacent_sum(data: &[i32]) {
    if let Some((min_sum, (a, b))) = min_adjacent_sum(data) {
        println!("Мінімальна пара: ({}, {}) з сумою {}", a, b, min_sum);
    } else {
        println!("Неможливо знайти мінімальну пару (вектор містить менше двох елементів).");
    }
}

fn main() {
    
    let random_vec = gen_random_vector(20);

    println!("Згенерований вектор: {:?}", random_vec);

    print_min_adjacent_sum(&random_vec);
}
