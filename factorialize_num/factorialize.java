public class factorialize {
  public static void main(String... args) {
    int res = factorializeCal(6);
    System.out.println(res);
  }

  public static int factorializeCal(int n) {
    return n <= 1 ? 1 :  n * factorializeCal(n-1);
  }
}
