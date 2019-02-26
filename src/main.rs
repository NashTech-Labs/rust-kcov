#[cfg_attr(tarpaulin, skip)]
fn main() {

    let number: u32 = 5;
    println!("Square of {} is {}", number, square_of_integer(number));
    println!("Cube of {} is {}", number, cube_of_integer(number));
}

fn square_of_integer(operand1: u32) -> u32
{
    let result = operand1 * operand1;
    return result;
}

fn cube_of_integer(operand1: u32) -> u32
{
    let result = operand1 * operand1 * operand1;
    return result;
}


    #[test]
    fn test_square_of_integer() {
        assert_eq!(square_of_integer(2), 4);
    }

    #[test]
        fn negative_test_square_of_integer() {
            assert_ne!(square_of_integer(3), 100);
        }

    #[test]
    fn test_cube_of_integer() {
        assert_eq!(cube_of_integer(11), 1331);
    }

    #[test]
        fn negative_test_cube_of_integer() {
            assert_ne!(cube_of_integer(5), 1225);
        }