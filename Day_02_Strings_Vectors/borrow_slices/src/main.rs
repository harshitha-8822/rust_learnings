

//input --> vector of integers
//output --> sum, average, max, min

fn sum(nums : &[i32]) -> i32{
    nums.iter().sum()
}
fn max(nums: &[i32]) -> Option<i32>{
    // returning option, because if slice if empty, there will be no max value
    nums.iter().copied().max()
    // either None or Some(val) is returned
}
fn min(nums: &[i32]) -> Option<i32>{
    nums.iter().copied().min()
}

fn average(nums: &[i32]) -> Option<f64>{
    if nums.is_empty(){
        None
    }
    else{
        Some(sum(nums) as f64/ nums.len() as f64)
    }
}
fn main(){
    let v = vec![10, 20, 30, 40, 50];

    println!("{}",sum(&v));
    println!("{:?}",min(&v));
    println!("{:?}",max(&v));
    println!("{:?}",average(&v));

}