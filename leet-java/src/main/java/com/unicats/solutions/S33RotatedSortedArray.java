package com.unicats.solutions;

public class S33RotatedSortedArray {
  
    public static void main(String[] args) {
      System.out.println(new S33RotatedSortedArray().search(new int[] {4, 5, 6, 1, 2, 3}, 2));
    }

    public int search(int[] nums, int target) {

      int left = 0;
      int right = nums.length - 1;
      int middle = (right + left) >> 1;

      while (left < right) {
        middle = (right + left) >> 1;
        int left_el = nums[left];
        int right_el = nums[right];
        int middle_el = nums[middle];

        if (left_el <= middle_el) {
          if (isBetween(left_el, middle_el, target)) {
            right = middle;
          } else {
            left = middle + 1;
          }
          
        } else { // if (middle_el < right_el) {
          if (isBetween(middle_el, right_el, target)) {
            left = middle + 1;
          } else {
            right = middle;
          }
        }
      }

      if (nums[left] != target)
        return -1;
      else return left;
    }

    private boolean isBetween(int left, int right, int target) {
      return (left <= target && target <= right);
    }
}
