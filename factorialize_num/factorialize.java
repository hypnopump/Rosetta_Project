public class factorialize {
  public static void main(String... args) {
    int res = factorializeCal(6);
    System.out.println(res);
  }

  public static int factorializeCal(int n) {
    if(n <= 1)
      return 1;
    else
      return n * factorializeCal(n-1);
  }
}
