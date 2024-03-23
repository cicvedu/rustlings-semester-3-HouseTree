// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // factorial_arrs.iter().flod(num, |acc, x| acc / x)

    if num == 0 {
        1
    }else {
        /*理解错误，以为是传值24求对应的阶乘 */
        // let mut factorial_num_vec: Vec<u64> = vec![];
        // factorial_num_vec.push(1 as u64);
        // factorial_num_vec.push((1..3).fold(1, |acc, x| acc*x) as u64);
        // factorial_num_vec.push((1..4).fold(1, |acc, x| acc*x) as u64);
        // factorial_num_vec.push((1..5).fold(1, |acc, x| acc*x) as u64);
        // factorial_num_vec.push((1..6).fold(1, |acc, x| acc*x) as u64);
        // match num{
        //     x if x ==  factorial_num_vec[0] => 1,
        //     x if x ==  factorial_num_vec[1] => 2,
        //     x if (x ==  6 as u64) => 3,
        //     x if x ==  factorial_num_vec[3] => 4,   
        //     x if x ==  factorial_num_vec[4] => 5,
        //     _ => 9999,
        //     // 0_u64..=u64::MAX => todo!(),
        // }
        (1..num + 1).fold(1, |acc, x| acc*x) as u64
    }
    


    // if num == 0 as u64 || num == 1 as u64 {
    //     return 1 as u64
    // }else {
    //     let mut num_d = num;
    //     let mut factorial_num: u64 = 2;
    //     while true {
    //         if num_d == factorial_num{
    //             break;
    //         }
    //         num_d = num_d / factorial_num;
    //         factorial_num += 1;
    //     }
    //     return factorial_num
    // }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_3() {
        assert_eq!(6, factorial(3));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    
}
