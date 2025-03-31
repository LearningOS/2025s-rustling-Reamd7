/*
	队列
	这个问题要求你使用队列来实现约瑟夫环问题
*/

use std::collections::VecDeque;

fn josephus(n: usize, k: usize) -> usize {
    // 创建一个队列来表示人的圈子
    let mut queue: VecDeque<usize> = (1..=n).collect();

    // 保持移除人，直到只剩一个
    while queue.len() > 1 {
        // 跳过 k-1 个人
        for _ in 0..k - 1 {
            //TODO
            let temp = queue.pop_front();
            if let Some(temp) = temp {
                queue.push_back(temp);
            } else {
                break;
            }
        }

        //TODO
        queue.pop_front();
    }

    // 最后剩下的人是获胜者
    queue.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_josephus() {
        assert_eq!(josephus(7, 3), 4);
        assert_eq!(josephus(10, 2), 5);
        assert_eq!(josephus(5, 1), 5);
        assert_eq!(josephus(1, 1), 1);
    }
}