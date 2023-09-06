use std::{alloc::{alloc, dealloc, handle_alloc_error, Layout}, ops::Deref};


pub struct Rc<T> {
    // 用于存储值和计数 
    value: T,
    rcount: i32,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        // 构造时让计数为1
        Rc {
            value,
            rcount: 1,
        }
    }

    pub fn inc_count(&mut self) -> i32 {
        // 增加计数
        self.rcount += 1;
        self.rcount
    }

    pub fn dec_count(&mut self) -> i32{
        // 减少计数
        self.rcount -= 1;
        self.rcount
    }
}

pub struct MyRc<T> {
    // 智能指针
    rct: *mut Rc<T>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        // 需要用到 unsafe 模式，新建的时候分配一块内存空间，用于存储值和计数
        unsafe {
            let layout = Layout::new::<Rc<T>>();
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            *(ptr as *mut Rc<T>) = Rc::<T>::new(value);
            MyRc::<T> {
                rct: ptr as *mut Rc<T>,
            }

        }
    }
    
    pub fn strong_count(&self) -> i32 {
        // 返回计数
        unsafe {
            (*(self.rct)).rcount
        }
    }

}

impl<T> Drop for MyRc<T> {
    // 实现离开作用域自动将计数减一，如果计数为0，就直接收回分配的空间
    fn drop(&mut self) {
        unsafe{
            if (*(self.rct)).dec_count() == 0 {
                let layout = Layout::new::<Rc<T>>();
                dealloc(self.rct as *mut u8, layout);
            } 
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        // 实现 clone trait
        unsafe{
            (*(self.rct)).inc_count();
            let new_rct = self.rct;
            MyRc::<T> {
                rct: new_rct,
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    // 实现解构
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{
            &(*(self.rct)).value
        }
    }
}