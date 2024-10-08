package com.unicats.solutions;

import java.util.List;
import java.util.ArrayList;

import com.unicats.solutions.common.TreeNode;

public class S94BTInorderTraversal {
  
  public static void main(String[] args) {
    TreeNode root = new TreeNode(1);
    root.right = new TreeNode(2);
    root.right.left = new TreeNode(3);
    System.out.println(new S94BTInorderTraversal().inorderTraversal(root));
  }

  public List<Integer> inorderTraversal(TreeNode root) {
    ArrayList<Integer> result = new ArrayList<>();
    processNode(root, result);
    return result;
  }
  
  private void processNode(TreeNode node, ArrayList<Integer> result) {
    if (node == null) return;
    processNode(node.left, result);
    result.add(node.val);
    processNode(node.right, result);
  }
}
