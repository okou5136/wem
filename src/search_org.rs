pub fn does_contain_vec(target: String, chars: Vec<char>) -> bool
{
    for tarchar in target.chars() {
        for single_char in &chars {
            if tarchar == *single_char {
                return true;
            }
        }
    }
    return false;
}
