public class HelloWorld {
  public static void main(String[] args) {
	int a = 7;
    System.out.println(fac(a));
  }

    public static int fac(int a) {
      if (a == 1) {
          return 1;
      } else {
          return a*fac(a-1);
      }
  }
}