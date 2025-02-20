/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    if intervals.len() < 2 {
        return intervals;
    }
    let mut intervals = intervals;
    intervals.sort_by(|a, b| {
        a[0].cmp(&b[0])
        //if a[0] == b[0] {
        //    a[1].cmp(&b[1])
        //} else {
        //    a[0].cmp(&b[0])
        //}
    });

    // 窗口前端推进的循环序号
    let mut win_it = 0;
    // 窗口扩展时的循环序号
    let mut win_extend_it = 0;
    while win_it < (intervals.len() - 1) {
        win_extend_it = win_it + 1;
        // 扩展窗口的end
        while win_extend_it < intervals.len() && intervals[win_it][1] >= intervals[win_extend_it][0] {
            // 扩展窗口
            if intervals[win_it][1] < intervals[win_extend_it][1] {
                intervals[win_it][1] = intervals[win_extend_it][1];
            }

            win_extend_it += 1;
        }
        win_extend_it -= 1;

        // 删除窗口覆盖的范围
        while win_extend_it > win_it {
            intervals.remove(win_extend_it);
            win_extend_it -= 1;
        }

        // 推进窗口的start
        win_it += 1;
    }

    intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
