pub mod vecc {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[macro_export]
macro_rules! vec {
( $( $x:expr ),* ) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = vecc::add(2, 2);
        assert_eq!(result, 4);
    }
}
