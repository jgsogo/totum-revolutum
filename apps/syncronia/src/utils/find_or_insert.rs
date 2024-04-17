pub fn mut_find_or_insert<T: PartialEq, P>(vec: &mut Vec<T>, predicate: P, val: T) -> (&mut T, bool)
where
    P: FnMut(&T) -> bool,
{
    if let Some(i) = vec.iter().position(predicate) {
        (&mut vec[i], false)
    } else {
        vec.push(val);
        (vec.last_mut().unwrap(), true)
    }
}
