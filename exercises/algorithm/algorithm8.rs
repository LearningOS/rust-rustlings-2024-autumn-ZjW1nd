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
            Ok(self.elements.remove(0usize))
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

pub struct myStack< T>
{
	//TODO，用两个队列实现一个栈，为什么两个队列？？？还有为什么不用vec自己的性质...
    cur: u8,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            cur: 1,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        match self.cur {
            1 => {
                self.q1.enqueue(elem);
            }
            2 => {
                self.q2.enqueue(elem);
            }
            _ => {}
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // 懂了，就是tmd每次pop的时候都把所有元素弹出到剩一个
        // 元素在两个队列中循环
        match self.cur {
            1 => {
                if self.q1.is_empty() {return Err("Stack is empty");}
                while self.q1.size() > 1 {
                    let elem = self.q1.dequeue().unwrap();
                    self.q2.enqueue(elem);
                    self.cur = 2;
                }
                Ok(self.q1.dequeue().unwrap())
            }
            2 => {
                if self.q2.is_empty() {return Err("Stack is empty");}
                while self.q2.size() > 1 {
                    let elem = self.q2.dequeue().unwrap();
                    self.q1.enqueue(elem);
                    self.cur = 1;
                }
                Ok(self.q2.dequeue().unwrap())
            }
            _ => {Err("Wrong cur")}
        }
		
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
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
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}