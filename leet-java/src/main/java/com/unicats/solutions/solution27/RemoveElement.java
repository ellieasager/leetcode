package com.unicats.solutions.solution27;

public class RemoveElement {

    public static void main(String[] args) {
      System.out.println(new RemoveElement().removeElement(new int[]{3,2,2,3}, 3));
    }

  public int removeElement(int[] nums, int val) {
    int k = 0;
    for (int x : nums) {
        if (x != val) {
            nums[k++] = x;
        }
    }
    return k;
  }

  public int removeElementEllie(int[] nums, int val) {
    int currentPostion = 0;
    
    while (currentPostion < nums.length) {
      if (nums[currentPostion] == val) {
        boolean foundGoodPlace = false;
        int insertPosition = currentPostion;
        while (insertPosition < nums.length - 1 && !foundGoodPlace) {
          insertPosition++;
          if (nums[insertPosition] != val) 
          { 
            foundGoodPlace = true;
            break;
          }
        }
        if (foundGoodPlace) swap(nums, currentPostion, insertPosition); 
        else break;
      }
      currentPostion++;
    }
    return currentPostion;
  }

  void swap(int[] nums, int a, int b) {
    int tmp = nums[a];
    nums[a] = nums[b];
    nums[b] = tmp;
  }

  // int tmp = nums[currentPostion];
  // nums[currentPostion] = nums[insertPosition];
  // nums[insertPosition] = tmp;
  
}
