fn sort<A,T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();
    array
}

#[cfg(test)]
mod tests {
    use crate::sort;
    #[test]
    fn it_sorts() {
        let a = [-5,4,1,-3,2];
        let sorted_a = sort(a);
        assert_eq!(a, [-5,4,1,-3,2]); // Why can I still refer to a?
        assert_eq!(sorted_a, [-5,-3,1,2,4]);
    }
}


// find . -not -path "./target/*" -not -path "./.git/*" | entr -rc cargo test --lib
