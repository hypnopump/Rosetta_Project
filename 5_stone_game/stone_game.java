public class stone_game {
  public static void main(String... args){
    int[] cases = {1,2,3,4,5,6,7,10};
    int[] moves = {2,3,5};
    String[] convert = {"Second", "First"};
    game(cases, convert, moves);
  }

  private static int p2(int c, int[] moves){
    int minResult = 1;
    for(int m: moves){
      if(c - m >= 0){
        int result = p1(c-m, moves);
        if(result < minResult)
          minResult = result;
      }
    }
    return minResult;
  }

  private static int p1(int c, int[] moves){
    int maxResult = 0;
    for(int m: moves){
      if(c - m >= 0){
        int result = p2(c - m, moves);
        if(result > maxResult)
          maxResult = result;
      }
    }
    return maxResult;
  }

  private static void game(int[] cases, String[] convert, int[] moves){
    for(int c: cases){
      int outcome = p1(c, moves);
      System.out.println(convert[outcome]);
    }
  }
}
