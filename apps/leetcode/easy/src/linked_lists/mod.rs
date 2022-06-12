// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// region:      --- METHODS ---

// region:      --- METHOD: Iteration ---

/// Returns a new `ListNode` without the given value by iterating over the list.
///
/// # Parameters
/// - `head: Option<Box<ListNode>>`: The head of the list.
/// - `val: i32`: The value to remove.
/// # Examples
/// ```rust
/// use leetcode::linked_lists::ListNode;
/// let mut head = ListNode::new(1);
/// head.next = Some(Box::new(ListNode::new(2)));
/// head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
/// head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
/// head.next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
/// let mut head = leetcode::linked_lists::remove_elements(&mut head, 3); // head: Some(Box { val: 1, next: Some(Box { val: 2, next: Some(Box { val: 4, next: Some(Box { val: 5, next: None }) }) }) })
/// let mut head = leetcode::linked_lists::remove_elements(&mut head, 1); // head: Some(Box { val: 2, next: Some(Box { val: 4, next: Some(Box { val: 5, next: None }) }) })
/// ```
/// # Panics
/// None
/// # Performance
/// ## Time Complexity
/// O(n)
/// ## Space Complexity
/// O(1)
/// ## Runtime
/// 5 ms, faster than 69.77% of Rust online submissions for Remove Linked List Elements.
/// ## Memory Usage
/// 2.8 MB, less than 79.07% of Rust online submissions for Remove Linked List Elements.
pub(crate) fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut nodes = None;
    let mut end = &mut nodes; // what does &mut do -> tip: &mut is not a reference, it is a mutable reference to a reference. SO who is the owner of this reference? The owner of the reference is the owner of the reference to the reference.

    while let Some(mut node) = head {
        // Takes the value out of the option, leaving a [None] in its place.
        head = node.next.take(); // what does this do? -> tip: take is a method on the Option enum.

        if node.val != val {
            // Inserts value into the option, then returns a mutable reference to it.
            end = &mut end.insert(node).next; // what does this do? -> inserts the node that is not equal to val into the end variable.
        } // end of if
    } // end of while loop -> while loop is done when head is None. and iterates over the head: Option<Box<ListNode>> until it is None.
    nodes // Return None if the while loop isn't triggered and returns the nodes.
} // end of remove_elements

// endregion:   --- METHOD: Iteration ---

// region:      --- METHOD: Recursion ---

/// Returns the length of the list with the target value removed
/// # Example
/// ```rust
/// use leetcode::linked_lists::remove_element;
/// let mut head = ListNode::new(1);
/// head.next = Some(Box::new(ListNode::new(2)));
/// head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
/// head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
/// head.next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
/// let res = remove_element(&mut head, 3);
/// assert_eq!(res, 2);
/// ```
/// # Performance
/// O(n)
/// # Space
/// O(1)
/// # Panics
///
/// Panics if .
pub(crate) fn remove_elements_recursion(
    head: Option<Box<ListNode>>,
    val: i32,
) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    } else {
        let mut unbox_head: Box<ListNode> = head.unwrap();
        let mut new_head: Option<Box<ListNode>> = remove_elements_recursion(unbox_head.next, val);

        if unbox_head.val == val {
            new_head = new_head;
        } else {
            unbox_head.next = new_head;
            new_head = Some(unbox_head);
        }

        return new_head;
    }
}

/**
 * Source: https://doc.rust-lang.org/std/mem/fn.take.html
What is Take () in Rust?
Replaces dest with the default value of T, returning the previous dest value.

If you want to replace the values of two variables, see swap.
If you want to replace with a passed value instead of the default value, see replace.

take allows taking ownership of a struct field by replacing it with an “empty” value. Without take you can run into issues like these: ⓘ struct Buffer<T> { buf: Vec<T> } impl<T> Buffer<T> { fn get_and_reset(&mut self) -> Vec<T> { // error: cannot move out of dereference of `&mut`-pointer let buf = self.


  But take can be used to disassociate the original value of self.buf from self, allowing it to be returned:
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

// region:      --- TEST SUITES ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        let _head: () = create_head_list_node(&mut head);

        let result = remove_elements_recursion(head, 6);
        let result_head = result.unwrap();
        assert_eq!(result_head.val, 1);
        assert_eq!(result_head.next.unwrap().val, 2);
    }

    /// Create a linked list with the given head node.
    /// # Parameters
    /// * `head: Option<Box<ListNode>>`
    /// # Returns
    /// * `()`
    /// # Panics
    /// - No panic.
    fn create_head_list_node(head: &mut Option<Box<ListNode>>) -> () {
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
    }

    // fn create_custom_head_list_nodes_with_presets(
    //     head: &mut Option<Box<ListNode>>,
    //     val: i32,
    //     next: Option<Box<ListNode>>,
    // ) -> () {
    //     head.as_mut().unwrap().val = val;
    //     head.as_mut().unwrap().next = next;
    // }
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

// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let mut borrow_head = head.clone();

//     if borrow_head == None {
//         return None;
//     } else {
//         // while Some(&mut borrow_head.unwrap()) == borrow_head.unwrap().next {
//         let mut node = borrow_head.clone();
//         while let Some(mut node) = borrow_head.clone() {
//             if node.val == val {
//                 node = node;
//                 println!("It's val");
//             } else {
//                 borrow_head.clone().unwrap().next = Some(node);
//                 println!("Not val");
//                 break;
//             }
//         }
//         return node.clone();
//     }
// } // end of fn remove_elements

// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let mut borrow_head = head.clone();

//     if borrow_head == None {
//         return None;
//     } else {
//         // let mut node: Box<ListNode> = borrow_head.unwrap().clone();

//         let mut node: Option<Box<ListNode>> = borrow_head.clone();

//         println!("node: {:?}", node.clone());
//         while let Some(mut node) = borrow_head.clone() {
//             if node.val == val {
//                 node = node;

//                 println!("It's val");
//             } else if Some(node.val) == None {
//                 return None;
//             } else {
//                 // node.next = borrow_head.clone().unwrap().next = Some(node);
//                 node.next = borrow_head.clone().unwrap().next;
//                 println!("node.next: {:?}", node.next.clone());
//                 println!("Not val");
//                 // break;
//             }
//         }
//         // return borrow_head.clone();
//         return node.clone();
//     }
// } // end of fn remove_elements

// pub fn rem_elem(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let borrow_head = head.clone();

//     if borrow_head == None {
//         return None;
//     } else {
//         // let mut node: = borrow_head.unwrap().clone();
//         let mut node: Box<ListNode> = borrow_head.unwrap().clone();

//         while node.val == val {
//             if node.val == val {
//                 //     node = node;
//                 // Line 31, Char 50: called `Option::unwrap()` on a `None` value (solution.rs)
//                 // use match to defer case of None to return None
//                 // node = node.next.unwrap().next.unwrap();
//                 node = {
//                     let this = node.next.unwrap().next;
//                     match this {
//                         Some(val) => val,
//                         None => {
//                             return None;
//                         }
//                     }
//                 };
//                 // in case of [7,7,7,7] stack overflow here, so if the next node.val = node -> node = node.next.unwrap().next;
//                 //  break;
//                 println!("It's val");
//             } else if Some(node.val) == None {
//                 return None;
//             } else {
//                 // node.next = borrow_head.clone().unwrap().next = Some(node);
//                 // node.next = borrow_head.clone().unwrap().next;
//                 node.next = node.next.unwrap().next;
//                 println!("node.next: {:?}", node.next.clone());
//                 println!("Not val");
//                 // break;
//                 // return node.next.clone();
//             }
//         }
//         // return borrow_head.clone();
//         return Some(node.clone());
//     }
// } // end of fn remove_elements

// pub fn rem_el(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let borrow_head: Option<Box<ListNode>> = head.clone();
//     let mut node: Box<ListNode> = Clone::clone(&match borrow_head {
//         Some(val) => val,
//         None => return None,
//     });

//     loop {
//         if (node.val) != val {
//             node.next = node.next.unwrap().next;
//             if node.next == None {
//                 return None;
//             } else {
//                 node = {
//                     match node.next {
//                         Some(val) => val,
//                         None => {
//                             return None;
//                         }
//                     }
//                 };
//             }
//         } else {
//             let list_node: Box<ListNode> = node.next.clone().unwrap();
//             if (node.val & list_node.val) == val {
//                 node = {
//                     match {
//                         match node.next {
//                             Some(val) => val,
//                             None => {
//                                 return None;
//                             }
//                         }
//                     } // end of match node.next
//                     .next
//                     {
//                         Some(val) => val,
//                         None => {
//                             return None;
//                         }
//                     } // end of match next
//                 }; // end of match ==> node.next = node.next.unwrap().next
//             } else if Some(node.val) == None {
//                 return None;
//             }
//             node.next = {
//                 match node.next {
//                     Some(val) => val,
//                     None => {
//                         return None;
//                     }
//                 }
//             } // end of match node.next
//             .next;
//         }
//         return Some(Clone::clone(&node));
//     }
// } // end of fn remove_elements
