package com.unicats.solutions.solution20;

import java.util.Stack;

public class ValidParentheses {

    public static void main(String[] args) {
      System.out.println(new ValidParentheses().isValid("()".toString()));
    }

      public boolean isValid(String s) {
        Stack<Character> stack = new Stack<>();
        for (char current : s.toCharArray()) {
            if (current == '(' || current == '[' || current == '{') {
                stack.push(current);
            } else {
                if (stack.empty() || !isMatch(stack.peek(), current)) {
                    return false;
                } else {
                    stack.pop();
                }
            }
        }
        return stack.empty();
    }

    private boolean isMatch(Character s1, Character s2) {
        return (
            s1 == '(' && s2 == ')' ||
            s1 == '[' && s2 == ']' ||
            s1 == '{' && s2 == '}'
        );
    }
}
