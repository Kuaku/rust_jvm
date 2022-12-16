public class HelloWorld {
  public static void main(String[] args) {
    int c = fac(5);
    System.out.println(c);
  }

    public static int fac(int value) {
      if (value == 1) {
          return 1;
      }
        return value*fac(value-1);
  }
}