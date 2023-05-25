import java.util.Scanner;

public class boj2609 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int m = sc.nextInt();

        // 최소공배수는 약수중에서 일치하는 가장 작은값

        // 유클리드 호제법
        // 두 수의 나머지가 0이 될때까지 계속해서 반복해서 나머지를 구하는 방식 (0이되면 그 수가 최소공배수이다)
        int gcd = GCD(n, m);
        System.out.println(gcd);
        System.out.println((n*m)/gcd);


    }

    public static int GCD(int x, int y){
        if(y == 0) {
            return x;
        }
        else {
            return GCD(y, x % y);
        }
    }

}
