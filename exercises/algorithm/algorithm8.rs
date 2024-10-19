/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/



#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

use std::collections::VecDeque;

pub struct MyStack<T> {
    q1: VecDeque<T>,
    q2: VecDeque<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: T) {
        // 将新元素压入 q1
        self.q1.push_back(x);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果 q1 为空且 q2 也为空，则队列为空
        if self.q1.is_empty() && self.q2.is_empty() {
            return Err("Queue is empty");
        }

        // 如果 q1 为空，则将 q2 中的元素移到 q1 中
        if self.q1.is_empty() {
            std::mem::swap(&mut self.q1, &mut self.q2);
        }

        // 将 q1 中的所有元素（除了最后一个）移到 q2 中
        while self.q1.len() > 1 {
            self.q2.push_back(self.q1.pop_front().unwrap());
        }

        // 弹出 q1 中的最后一个元素
        self.q1.pop_front().ok_or("Queue is empty")
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

// impl<T> MyStack<T> {
//     pub fn new() -> Self {
//         Self {
//             q1: Queue::new(),
//             q2: Queue::new(),
//         }
//     }

//     pub fn push(&mut self, elem: T) {
//         // Always push to q1
//         self.q1.enqueue(elem);
//     }


//     // pub fn pop(&mut self) -> Result<T, &str> {
//     //     // 如果 q1 为空且 q2 也为空，则队列为空
//     //     if self.q1.is_empty() && self.q2.is_empty() {
//     //         return Err("Queue is empty");
//     //     }
    
//     //     // 如果 q1 为空，则将 q2 中的元素移到 q1 中
//     //     if self.q1.is_empty() {
//     //         std::mem::swap(&mut self.q1, &mut self.q2);
//     //     }
    
//     //     // 从 q1 中 dequeue
//     //     self.q1.dequeue()
//     // }

//     // pub fn pop(&mut self) -> Result<T, &str> {
//     //     // 尝试从 q1 中 dequeue
//     //     if let Ok(temp_result) = self.q1.dequeue() {
//     //         // 交换 q1 和 q2
//     //         std::mem::swap(&mut self.q1, &mut self.q2);
//     //         Ok(temp_result)
//     //     } else {
//     //         Err("Queue is empty")
//     //     }
//     // }
    
//     // pub fn pop(&mut self) -> Result<T, &str> {
//     //     // 先尝试从 q1 中 dequeue
//     //     let result = {
//     //         let temp_result = self.q1.dequeue();
//     //         if temp_result.is_ok() {
//     //             temp_result
//     //         } else {
//     //             return Err("Queue is empty");
//     //         }
//     //     };
    
//     //     // 然后交换 q1 和 q2
//     //     std::mem::swap(&mut self.q1, &mut self.q2);
    
//     //     result
//     // }
    
//     // pub fn pop(&mut self) -> Result<T, &str> {
//     //     // Transfer all elements except the last one from q1 to q2
//     //     while self.q1.size() > 1 {
//     //         if let Ok(element) = self.q1.dequeue() {
//     //             self.q2.enqueue(element);
//     //         }
//     //     }

//     //     // The last element in q1 is the top of the stack
//     //     let result = self.q1.dequeue();

//     //     // Swap q1 and q2
//     //     std::mem::swap(&mut self.q1, &mut self.q2);

//     //     result
//     // }

//     // pub fn is_empty(&self) -> bool {
//     //     self.q1.is_empty() && self.q2.is_empty()
//     // }
// }

#[cfg(test)]
mod tests {
    use super::MyStack;

    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Queue is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Queue is empty"));
        assert_eq!(s.is_empty(), true);
    }
}


// #[derive(Debug)]
// pub struct Queue<T> {
//     elements: Vec<T>,
// }

// impl<T> Queue<T> {
//     pub fn new() -> Queue<T> {
//         Queue {
//             elements: Vec::new(),
//         }
//     }

//     pub fn enqueue(&mut self, value: T) {
//         self.elements.push(value)
//     }

//     pub fn dequeue(&mut self) -> Result<T, &str> {
//         if !self.elements.is_empty() {
//             Ok(self.elements.remove(0))
//         } else {
//             Err("Queue is empty")
//         }
//     }

//     pub fn peek(&self) -> Result<&T, &str> {
//         match self.elements.first() {
//             Some(value) => Ok(value),
//             None => Err("Queue is empty"),
//         }
//     }

//     pub fn size(&self) -> usize {
//         self.elements.len()
//     }

//     pub fn is_empty(&self) -> bool {
//         self.elements.is_empty()
//     }
// }

// impl<T> Default for Queue<T> {
//     fn default() -> Queue<T> {
//         Queue {
//             elements: Vec::new(),
//         }
//     }
// }

// pub struct MyStack<T> {
//     q1: Queue<T>,
//     q2: Queue<T>,
// }

// impl<T> MyStack<T> {
//     pub fn new() -> Self {
//         Self {
//             q1: Queue::new(),
//             q2: Queue::new(),
//         }
//     }

//     pub fn push(&mut self, elem: T) {
//         // Always push to q1
//         self.q1.enqueue(elem);
//     }

//     pub fn pop(&mut self) -> Result<T, &str> {
//         // Transfer all elements except the last one from q1 to q2
//         while self.q1.size() > 1 {
//             if let Ok(element) = self.q1.dequeue() {
//                 self.q2.enqueue(element);
//             }
//         }

//         // The last element in q1 is the top of the stack
//         let result = self.q1.dequeue();

//         // Swap q1 and q2
//         std::mem::swap(&mut self.q1, &mut self.q2);

//         result
//     }

//     pub fn is_empty(&self) -> bool {
//         self.q1.is_empty() && self.q2.is_empty()
//     }
// }

// #[cfg(test)]
// mod tests {
// 	// use super::*;
// 	use super::MyStack;
// 	#[test]
// 	fn test_queue(){
// 		let mut s = myStack::<i32>::new();
// 		assert_eq!(s.pop(), Err("Stack is empty"));
//         s.push(1);
//         s.push(2);
//         s.push(3);
//         assert_eq!(s.pop(), Ok(3));
//         assert_eq!(s.pop(), Ok(2));
//         s.push(4);
//         s.push(5);
//         assert_eq!(s.is_empty(), false);
//         assert_eq!(s.pop(), Ok(5));
//         assert_eq!(s.pop(), Ok(4));
//         assert_eq!(s.pop(), Ok(1));
//         assert_eq!(s.pop(), Err("Stack is empty"));
//         assert_eq!(s.is_empty(), true);
// 	}
// }