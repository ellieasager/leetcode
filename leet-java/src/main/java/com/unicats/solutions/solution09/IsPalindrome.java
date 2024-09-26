package com.unicats.solutions.solution09;

public class IsPalindrome {

    public static void main(String[] args) {
      System.out.println(new IsPalindrome().isPalindrome(0));
    }

    public boolean isPalindrome(int x) {
      String lToRight = String.valueOf(x);
      String rToLeft = new StringBuilder(lToRight).reverse().toString();
      return lToRight.equals(rToLeft);
  }
}
