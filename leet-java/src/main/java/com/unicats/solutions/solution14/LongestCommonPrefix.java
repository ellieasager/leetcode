package com.unicats.solutions.solution14;

import java.util.Arrays;

public class LongestCommonPrefix {


    public static void main(String[] args) {
      // System.out.println(new LongestCommonPrefix().longestCommonPrefix(new String[]{"flower","flow","flight"}));
      System.out.println(new LongestCommonPrefix().longestCommonPrefix(new String[]{"ab","a"}));
    }

  public String longestCommonPrefix(String[] strs) {
    int n = strs.length;
    for (int i = 0; i < strs[0].length(); ++i) {
        for (int j = 1; j < n; ++j) {
            if (strs[j].length() <= i || strs[j].charAt(i) != strs[0].charAt(i)) {
                return strs[0].substring(0, i);
            }
        }
    }
    return strs[0];
  }

  public String longestCommonPrefixEllie(String[] strs) {

    // sort strings by length
    Arrays.sort(strs, (s1, s2) -> Integer.compare(s1.length(), s2.length()));

    String prefix = strs[0];
    boolean found = false;

    while (prefix.length() > 0 && !found) {
      found = true;
      for (String s: strs) {
        if (!s.startsWith(prefix)) {
          found = false;
          break;
        }
      }
      if (found) return prefix;
      prefix = prefix.substring(0, prefix.length() - 1);
    }
        return found ? prefix : "";
  }
  
}
