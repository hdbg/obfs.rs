#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        obfs_macro::confuse_flow!({
            let mut a = 123;
            a += 233;
        });
    }
}
