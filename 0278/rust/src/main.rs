struct Solution;

#[allow(dead_code)]
impl Solution {
   pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left < right {
            let middle = left + (right - left) / 2;

            if self.isBadVersion(middle) {
                right = middle;
            } else {
                left = middle + 1;
            }

        }

        return left;
    } 


    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        println!("{}", version);
        unimplemented!()
    }
}

fn main() {
    println!("Hello World!");
}
