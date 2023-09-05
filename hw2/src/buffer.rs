pub struct Buffer<T: std::ops::Add<Output = T> + Clone>
// 这里需要保证 T 是可以相加的，且允许克隆
{
    data: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Clone> Buffer<T> {
    pub fn new(data: Vec<T>) -> Buffer<T> {
        // 创建一个新的 Buffer
        Buffer { data }
    }
    pub fn sum(&self) -> Option<T> {
        // 返回类型是一个 Option
        if self.data.is_empty() {
            // 如果 Buffer 是空的，那么直接返回 None
            None
        } else {
            // 否则将其中所有项相加
            let mut sum = self.data[0].clone();
            for i in 1..self.data.len() {
                sum = sum + self.data[i].clone();
            }
            Some(sum)
        }
    }
}
