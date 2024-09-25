package com.unicats.solutions.solution01;

import java.util.HashMap;
import java.util.Map;

public class TwoSum {


    public static void main(String[] args) {
      int[] result = new TwoSum().twoSum(new int[] {1, 2, 3}, 4);
      // System.out.println(new TwoSum().twoSum(new int[] {1, 2, 3}, 4));
      System.out.println("[" + result[0] + "," + result[1] + "]");
    }

    public int[] twoSum(int[] nums, int target) {

      Map<Integer, Integer> m = new HashMap<>();

      for (int i=0; i < nums.length; i++) {
        int current_num = nums[i];
        int partner = target - current_num;
        if (m.containsKey(partner)) {
          return new int[] {i, m.get(partner)};
        } else {
          m.put(current_num, i);
        }
      }
      return new int[] {-1, -1};
    }
}
