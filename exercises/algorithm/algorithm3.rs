/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T : PartialOrd>(array: &mut [T]){
	// 快排非递归
    if array.len() <= 1 {return;}
    let mut stack = vec![(0, array.len() - 1)];// 用一个栈模拟递归，算是递归通用解法
    while let Some((left, right)) = stack.pop() {
        if left >= right { continue; }
        let pivot_index = partition(array, left, right);
        if pivot_index > 0 {
            stack.push((left, pivot_index - 1));
        }
        stack.push((pivot_index + 1, right));
    }

fn partition<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let pivot = right;
    let mut i = left;
    for j in left..right {
        if array[j] < array[pivot] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, pivot);
    i
}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}