// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/*
Before writing any code, it's good to make a list of edge cases that we need to consider. This is so that we can be certain that we're not overlooking anything while coming up with our algorithm, and that we're testing all special cases when we're ready to test. These are the edge cases that I came up with.

    The linked list is empty, i.e. the head node is None.
    Multiple nodes with the target value in a row.
    The head node has the target value.
    The head node, and any number of nodes immediately after it have the target value.
    All of the nodes have the target value.
    The last node has the target value.
*/

// impl Solution {

/// remove_elements takes a head node and a target value.
/// Function to find the node with the target value.
///
/// # Parameters
/// * `head: Option<Box<ListNode>>`
/// * `x: i32`
/// # Returns
/// * `Option<Box<ListNode>>`
///
/// # Edge Cases
/// 1. Empty lists's head node returns None.
/// 2. What if consecutive nodes have target value?
/// 3. The target value is the head node.
/// 4. The target value is the last node.
/// 5. Just after the head node, all nodes have the target value.
/// 6. All nodes have the target value.
/// 7. Only the last node has the target value.
///
/// # Constraints
/// 1. The linked list is not null. The number of nodes in the list is in the range [0, 104].
/// 2. The Node values are 1 <= Node.val <= 50
/// 3. The val is between 0 <= val <= 50

/// # Similar Paradigms
/// 1. [Hash Table](https://leetcode.com/problems/linked-list-cycle-ii/)
/// 2. [Two Pointers](https://leetcode.com/problems/linked-list-cycle-ii/)
/// # Related Topics
/// 1. Linked List
/// 2. Recursion
/// # Source
/// URL: [<a href="https://leetcode.com/problems/linked-list-cycle-ii/">LeetCode</a>]
/// title: 203. Remove Linked List Elements
/// # Example
/// 1. When val is 6, and the head is [1,2,6,3,4,5,6], the result should be [1,2,3,4,5].
/// ```rust
/// Input: head = [1,2,6,3,4,5,6], val = 6
/// Output: [1,2,3,4,5]
/// ```
/// 2. When the linked list is empty, return None.
/// ```rust
/// Input: head = [], val = 1
/// Output: []
/// ```
/// 3. When the head node has the target value, return the head node.
/// ```rust
/// Input: head = [1,2,3,4,5], val = 1
/// Output: [1,2,3,4,5]
/// ```
/// Given the head of a linked list and an integer val,
/// remove all the nodes of the linked list that has Node.val == val, and return the new head.

/// remove_elements takes a head node and a target value.
/// # Performance
/// Runtime: 10 ms, faster than 16.28% of Rust online submissions for Remove Linked List Elements.
/// Memory Usage: 2.9 MB, less than 39.53% of Rust online submissions for Remove Linked List Elements.
/// # Description
/// Given a linked list and a value, remove all occurrences of that value in the linked list.
/// # Panics
/// - No panic.
/// # Examples
/// 1. When val is 6, and the head is [1,2,6,3,4,5,6], the result should be [1,2,3,4,5].
/// ```rust
/// Input: head = [1,2,6,3,4,5,6], val = 6
/// Output: [1,2,3,4,5]
/// ```
/// 2. When the linked list is empty, return None.
/// ```rust
/// Input: head = [], val = 1
/// Output: []
/// ```
/// 3. When the head node has the target value, return the head node.
/// ```rust
/// Input: head = [1,2,3,4,5], val = 1
/// Output: [1,2,3,4,5]
/// ```
/// Given the head of a linked list and an integer val,
/// remove all the nodes of the linked list that has Node.val == val, and return the new head.
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    } else {
        let mut unbox_head: Box<ListNode> = head.unwrap();
        let mut new_head = remove_elements(unbox_head.next, val);
        if unbox_head.val == val {
            new_head = new_head;
        } else {
            unbox_head.next = new_head;
            new_head = Some(unbox_head);
        }
        return new_head;
    }
}

// region:      --- TEST SUITES ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(3)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(4)));

        let result = remove_elements(head, 6);
        let result_head = result.unwrap();
        assert_eq!(result_head.val, 1);
        assert_eq!(result_head.next.unwrap().val, 2);
    }
}

// endregion:   --- TEST SUITES ---
/*
 ________  ________  ________  ___  ___  ___  ___      ___ _______
|\   __  \|\   __  \|\   ____\|\  \|\  \|\  \|\  \    /  /|\  ___ \
\ \  \|\  \ \  \|\  \ \  \___|\ \  \\\  \ \  \ \  \  /  / | \   __/|
 \ \   __  \ \   _  _\ \  \    \ \   __  \ \  \ \  \/  / / \ \  \_|/__
  \ \  \ \  \ \  \\  \\ \  \____\ \  \ \  \ \  \ \    / /   \ \  \_|\ \
   \ \__\ \__\ \__\\ _\\ \_______\ \__\ \__\ \__\ \__/ /     \ \_______\
    \|__|\|__|\|__|\|__|\|_______|\|__|\|__|\|__|\|__|/       \|_______|
*/

// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     if head.clone().is_none() {
//         None
//     } else {
//         // https://stackoverflow.com/questions/70374050/cannot-find-function-in-this-scope-not-found-in-this-scope-in-rust
//         head.clone().unwrap().next = remove_elements(head.clone().unwrap().next, val);
//         if head.clone().unwrap().val == val {
//             return head.clone().unwrap().next;
//         } else {
//             return head.clone();
//         }
//     }
// }

// pub(crate) fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let mut faux_head = ListNode::new(-1);
//     faux_head.next = head; // output: [1,2,6,3,4,5,6] // target: 6
//     let mut current_node = faux_head.clone();

//     while current_node.next != None {
//         if {
//             let this = current_node.next.clone();
//             match this {
//                 Some(val) => val,
//                 None => break,
//             }
//         }
//         .val == val
//         {
//             current_node.next = current_node.next.unwrap().next;
//         } else {
//             current_node = *current_node.next.unwrap();
//         }
//     }

// pub(crate) fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let mut head: Option<Box<ListNode>> = head;
//     let mut prev: Option<Box<ListNode>> = None;
//     while let Some(node) = head.clone() {
//         if node.val == val {
//             if prev.is_none() {
//                 head = node.next;
//             } else {
//                 prev.as_mut().unwrap().next = node.next;
//             }
//         } else {
//             prev = Some(node.clone());
//             head = node.next;
//         }
//     }
//     head
// }
//     // let result = faux_head.next;
//     return faux_head.next;
// } // end of remove_elements

// } // end of Solution

// cspell:disable
/*

function removeElements(head: ListNode | null, val: number): ListNode | null {
  while (head && head.val == val) head = head.next;
  let curr = head;
  while (curr && curr.next) {
    if (curr.next.val === val) {
      curr.next = curr.next.next;
    } else {
      curr = curr.next;
    }
  }
  return head;
}


*/
