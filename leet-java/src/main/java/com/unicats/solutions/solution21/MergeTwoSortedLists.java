package com.unicats.solutions.solution21;

import java.util.Stack;

import com.unicats.solutions.common.ListNode;


public class MergeTwoSortedLists {

    public static void main(String[] args) {
    
        ListNode ln3= new ListNode(3);
        ListNode ln4 = new ListNode(4);
        ListNode ln1= new ListNode(1, ln3);
        ListNode ln2 = new ListNode(2, ln4);
        ListNode result = new MergeTwoSortedLists().mergeTwoLists(ln1, ln2);

      System.out.println(result);
    }

    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode dummy = new ListNode();
        ListNode curr = dummy;
        while (list1 != null && list2 != null) {
            if (list1.val <= list2.val) {
                curr.next = list1;
                list1 = list1.next;
            } else {
                curr.next = list2;
                list2 = list2.next;
            }
            curr = curr.next;
        }
        curr.next = list1 == null ? list2 : list1;
        return dummy.next;
    }

    public ListNode mergeTwoListsEllie(ListNode list1, ListNode list2) {

        if (list1 == null) return list2;
        if (list2 == null) return list1;
        // At this point, both aren't null!

        Stack<ListNode> s = new Stack<>();

        while (list1 != null || list2 != null) {

            if (list1 == null) {
                addToStack(s, list2);
                break;
            }
            if (list2 == null) {
                addToStack(s, list1);
                break;
            }

            if (list1.val <= list2.val) {
                s.push(list1);
                list1 = list1.next;
            } else {
                s.push(list2);
                list2 = list2.next;
            }
        }

        return findHead(s);
    }

    private void addToStack(Stack<ListNode> s, ListNode l) {
        while (l != null) {
            s.push(l);
            l = l.next;
        }
    }

    private ListNode findHead(Stack<ListNode> s) {

        ListNode head = null;
        ListNode previous = null;
        while (!s.empty()) {
            head = s.pop();
            head.next = previous;
            previous = head;
        }
        return head;
    }
}