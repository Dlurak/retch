pub fn fill_til_length<T: Clone + Copy>(vect: Vec<T>, length: usize, fill_item: T) -> Vec<T> {
    if vect.len() >= length {
        return vect;
    }

    let mut new_vect = vect.clone();
    new_vect.push(fill_item);

    fill_til_length(new_vect, length, fill_item)
}

pub fn create_pairs<T: Copy, U: Copy>(vect1: Vec<T>, vect2: Vec<U>) -> Vec<(Option<T>, Option<U>)> {
    let mut i = 0;
    let mut result: Vec<(Option<T>, Option<U>)> = vec![];

    loop {
        result.push((
            vect1.get(i).map(|&x| x),
            vect2.get(i).map(|&x| x),
        ));

        i += 1;
        if i >= vect1.len() && i >= vect2.len() {
            break;
        }
    }

    result
}
