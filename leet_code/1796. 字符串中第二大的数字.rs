// https://leetcode.cn/problems/second-largest-digit-in-a-string/
// 给你一个混合字符串 s ，请你返回 s 中 第二大 的数字，如果不存在第二大的数字，请你返回 -1 。

// 混合字符串 由小写英文字母和数字组成。

// 示例 1：
// 输入：s = "dfa12321afd"
// 输出：2
// 解释：出现在 s 中的数字包括 [1, 2, 3] 。第二大的数字是 2 。

// 示例 2：
// 输入：s = "abc1111"
// 输出：-1
// 解释：出现在 s 中的数字只包含 [1] 。没有第二大的数字。

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut first_highest = -1;
        let mut second_highest = -1;

        for char in s.chars() {
            if char.is_alphabetic() {
                continue;
            }
            if char.is_ascii_digit() {
                let current = char.to_digit(10).unwrap() as i32;
                if first_highest == -1 {
                    first_highest = current;
                } else if first_highest < current {
                    second_highest = first_highest;
                    first_highest = current;
                } else if first_highest > current && (second_highest == -1 || current > second_highest) {
                    second_highest = current;
                }
            }
        }
        return second_highest;
    }
}