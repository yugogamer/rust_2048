use rand::Rng;

const ARRAY_LENGTH: usize = 16;

enum Direction {
    TOP,
    LEFT,
    RIGHT,
    BOTTOM,
}

fn main() {
    let mut cases: [Option<u16>; ARRAY_LENGTH] = [None; ARRAY_LENGTH];
    add_random_cube(&mut cases);
    println!("{:?}", cases);
}

/// Add a random cube to the array
fn add_random_cube(cases: &mut [Option<u16>; ARRAY_LENGTH]) {
    let mut rng = rand::thread_rng();
    let mut none_array = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        if case.is_none() {
            none_array.push(index);
        }
    }
    let index = rng.gen_range(0..none_array.len());
    cases[none_array[index]] = Some(generate_random_value());
}

/// return 2 with 80% or 4 with 20% chance
fn generate_random_value() -> u16 {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(1..10);
    if value <= 8 {
        return 2;
    } else {
        return 4;
    }
}

fn moveCase(cases: &mut [Option<u16>; ARRAY_LENGTH], direction: Direction) {
    match direction {
        Direction::TOP => {
            for i in 0..cases.len() {
                if let Some(case) = cases[i] {
                    if let Some(next_case_index) = get_index_for_direction(i, &direction) {
                        match cases[next_case_index] {
                            Some(next_case) => {
                                if next_case == case {
                                    cases[next_case_index] = Some(next_case + case);
                                    cases[i] = None;
                                }
                            }
                            None => {
                                cases[next_case_index] = Some(case);
                                cases[i] = None;
                            }
                        }
                    }
                }
            }
        }
        Direction::LEFT => {
            for i in 0..cases.len() {
                if let Some(case) = cases[i] {
                    if let Some(next_case_index) = get_index_for_direction(i, &direction) {
                        match cases[next_case_index] {
                            Some(next_case) => {
                                if next_case == case {
                                    cases[next_case_index] = Some(next_case + case);
                                    cases[i] = None;
                                }
                            }
                            None => {
                                cases[next_case_index] = Some(case);
                                cases[i] = None;
                            }
                        }
                    }
                }
            }
        }
        Direction::RIGHT => {
            for i in 0..cases.len() {
                if let Some(case) = cases[i] {
                    if let Some(next_case_index) = get_index_for_direction(i, &direction) {
                        match cases[next_case_index] {
                            Some(next_case) => {
                                if next_case == case {
                                    cases[next_case_index] = Some(next_case + case);
                                    cases[i] = None;
                                }
                            }
                            None => {
                                cases[next_case_index] = Some(case);
                                cases[i] = None;
                            }
                        }
                    }
                }
            }
        }
        Direction::BOTTOM => {
            for i in cases.len()..0 {
                if let Some(case) = cases[i] {
                    if let Some(next_case_index) = get_index_for_direction(i, &direction) {
                        match cases[next_case_index] {
                            Some(next_case) => {
                                if next_case == case {
                                    cases[next_case_index] = Some(next_case + case);
                                    cases[i] = None;
                                }
                            }
                            None => {
                                cases[next_case_index] = Some(case);
                                cases[i] = None;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// send back index for the case in the given direction
/// # Argument
/// index : index where we start
/// direction : direction we want
/// # Return
///  and option of the index
fn get_index_for_direction(index: usize, direction: &Direction) -> Option<usize> {
    match direction {
        Direction::TOP => {
            let index = index as i32 - 4;
            if index < 0 {
                return None;
            } else {
                return Some(index as usize);
            }
        }
        Direction::LEFT => {
            let index = index as i32 - 1;
            if index >= 16 || (index + 1) % 4 == 0 {
                return None;
            } else {
                return Some(index as usize);
            }
        }
        Direction::RIGHT => {
            let index = index + 1;
            if index >= 16 || index % 4 == 0 {
                return None;
            } else {
                return Some(index);
            }
        }
        Direction::BOTTOM => {
            let index = index as i32 + 4;
            if index >= 16 {
                return None;
            } else {
                return Some(index as usize);
            }
        }
    }
}
