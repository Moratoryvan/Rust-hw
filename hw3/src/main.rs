pub mod stack;
pub mod hash_map;
pub mod MyRc;

#[cfg(test)]
mod tests {
    use crate::{MyRc::MyRc, stack::SimpleStack, hash_map};

    

    #[test]
    fn test_hash_map(){
        let map = hash_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6
        };

        assert_eq!(map["one"],1);
        assert_eq!(map["two"],2);
        assert_eq!(map["three"],3);
    }



    #[test]
    fn test_stacks() {
        let stack = SimpleStack::<i32>::new(); 
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(),Some(3));
        assert_eq!(stack.pop(),Some(2));

        stack.push(4);

        assert_eq!(stack.pop(),Some(4));
        assert_eq!(stack.pop(),Some(1));
        assert_eq!(stack.pop(),None);
    }

    #[test]
    fn test_MyRc(){
        let five = MyRc::new(5);
        assert_eq!(five.strong_count(),1);
        {
            let new_five = MyRc::clone(&five);
            assert_eq!(new_five.strong_count(),2);
            assert_eq!(*new_five,5);
        }
        assert_eq!(five.strong_count(),1);
        assert_eq!(*five,5);
    }
}
fn main(){}