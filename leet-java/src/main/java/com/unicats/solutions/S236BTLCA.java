package com.unicats.solutions;

import com.unicats.solutions.common.TreeNode;

public class S236BTLCA {
    public static void main(String[] args) {
      TreeNode node0 = new TreeNode(0);
      TreeNode node1 = new TreeNode(1);
      TreeNode node2 = new TreeNode(2);
      TreeNode node3 = new TreeNode(3);
      TreeNode node4 = new TreeNode(4);
      TreeNode node5 = new TreeNode(5);
      TreeNode node6 = new TreeNode(6);
      TreeNode node7 = new TreeNode(7);
      TreeNode node8 = new TreeNode(8);
      node2.left = node7;
      node2.right = node4;
      node5.left = node6;
      node5.right = node2;
      node1.left = node0;
      node1.right = node8;
      node3.left = node5;
      node3.right = node1;

      System.out.println(new S236BTLCA().lowestCommonAncestor(node3, node5, node4).val);
    }

    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
      if (root == null || root == p || root == q) {
        return root;
    }
    TreeNode left = lowestCommonAncestor(root.left, p, q);
    TreeNode right = lowestCommonAncestor(root.right, p, q);

    if (left != null && right != null) {
        return root; // Both nodes found, LCA is the current node
    } 
    else if (left != null) {
        return left; // p or q found in left subtree
    } 
    else if (right != null) {
        return right; // p or q found in right subtree
    } 
    else {
        return null; // Neither p nor q found
    }
    }
}
