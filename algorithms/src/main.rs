use std::{num, hash, collections::HashSet};

fn main() {
    println!("Hello, world!");


    //Algo 0: Mess around with Vectors and the vec!
    let mut numbers = vec![1,2,3,4,5,6,7,8,9,10];
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    //Algo 1: Print even and odd numbers...
    even_and_odd_numers(&mut numbers);

    //Algo 2: binary Search with Rust
    let array = [1,2,3,4,5];
    let target = 4;
    let _index = binary_search_in_rust(&array, target);
    println!("Binary Search index {}", _index.unwrap());


    //Algo 3: Get Unqiue Chars in a string 
    println!("");
    println!("");
    let my_string = "Hello World";
    let unique_string = get_unique_chars(&my_string);
    println!("Unique characters from {my_string} is = {}", unique_string.unwrap());

    
    

}

fn get_unique_chars(my_string: &str) -> Option<String> {
    let mut result = String::new();
    let mut hashSet = HashSet::new();


    for c in my_string.trim().replace(" ", "").chars(){
        hashSet.insert(c);
        
    }
    for c in hashSet{
        result.push(c);
    }

    return Some(result);

}


fn binary_search_in_rust(array: &[i32], target: i32) -> Option<usize>{
    let mut low = 0;
    let mut high = array.len() -1;

    while low <= high{ //until now when we know that the index of target is not here at all
        let mid = (low + high ) / 2;

        if array[mid] == target {
            return Some(mid)
        }else if array[mid] <= target {
            low = mid + 1;
        }else{
            high = mid -1;
        }

    }
    None
}

fn even_and_odd_numers(numbers: &mut Vec<i32>){

    for i in &mut *numbers{
        if *i % 2 == 0{
            println!("Number {} is even", i)
        }else{
            println!("Number {} is odd", i)
        }
    }
    let newvec = numbers;
    println!("Printing first vector (list) => {:?}", newvec);




   
}
