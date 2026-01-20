fn get_index(a: [i8; 5]) -> usize {
    return a.len() - 1;
}

pub fn mutable_arrays() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[get_index(a)] = 0;
    println!("a: {a:?}");
}

pub fn immutable_arrays() {
    let a = [5, 4, 3, 2, 1];
    println!("a: {a:?}");
}

pub fn array_iteration() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
    println!("All numbers are prime!");
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    print!("Transposing matrix:\n");
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

pub fn nested_arrays() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("Original matrix:");
    for row in &matrix {
        println!("{:?}", row);
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in &transposed {
        println!("{:?}", row);
    }
}
