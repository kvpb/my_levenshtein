//extern crate unicode_segmentation;

//use std::process;
//use unicode_segmentation::UnicodeSegmentation; //use std::string;
//use std::convert::TryFrom;

fn my_levenshtein(s_1: &String, s_2: &String) -> i32 //fn my_levenshtein(s1: &String, s2: &String) -> i32 {
{
    let l_1 = s_1.len(); //s_1.graphemes(true).count(); //s_1.chars().count(); //s_1.len();
    let l_2 = s_2.len();
    let mut i = 0;
    let mut j = 0;
    
    if l_2 != l_1
    {
        return -1;
    }
    while i < l_1
    {
        if s_2.chars().nth( i ).unwrap() != s_1.chars().nth( i ).unwrap() //if s_2.graphemes(true).nth( i ).unwrap() != s_1.graphemes(true).nth( i ).unwrap() //if UnicodeSegmentation::graphemes(&s_2, true).nth( i ).unwrap() != UnicodeSegmentation::graphemes(&s_1, true).nth( i ).unwrap() //if s_2[ i ] != s_1[ i ] // They deprecated graphemes()...
        {
            j += 1;
        }
        i += 1; //i++; // I hate bullshit. I abhor bullshit. I execrate bullshit. I despise bullshit. I detest bullshit. I vilipend bullshit. I loathe bullshit. I abominate bullshit. Les bullshitters sont des baltringues. Fuck bullshitters.
    }
    return j; //return i32::try_from(j).ok();
}

/*fn main()
{
    let s_0_1 = String::from("GGACTGA"); //let s_0_1: String = "GGACTGA"; //let s_0_1 = "GGACTGA";
    let s_0_2 = String::from("GGACTGA");
    let s_1_1 = String::from("ACCAGGG");
    let s_1_2 = String::from("ACTATGG");
    let s_2_1 = String::from("GGACGGATTCTG");
    let s_2_2 = String::from("AGG");
    let s_3_1 = String::from("");
    let s_3_2 = String::from("");
    
    format!("{}", my_levenshtein( &s_0_1, &s_0_2 ) );
    format!("{}", my_levenshtein( &s_1_1, &s_1_2 ) );
    format!("{}", my_levenshtein( &s_2_1, &s_2_2 ) );
    format!("{}", my_levenshtein( &s_3_1, &s_3_2 ) );
    //std::process::exit(code: i32);
}*/
