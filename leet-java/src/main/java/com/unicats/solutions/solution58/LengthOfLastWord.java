package com.unicats.solutions.solution58;

public class LengthOfLastWord {  
      public static void main(String[] args) {
        System.out.println(new LengthOfLastWord().lengthOfLastWord("a"));
      }
  
      public int lengthOfLastWord(String s) {
        String trimmed = s.trim();
        int spaceIdx = trimmed.lastIndexOf(' ');
        String lastWord = trimmed.substring(spaceIdx+1);
        return lastWord.length();
      }
  }
  
