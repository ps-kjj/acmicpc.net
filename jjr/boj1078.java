import java.util.Scanner;

public class boj1078 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int t = sc.nextInt();
        int cnt = 0;
        for (int i = 0; i < t; i++) {
            int n = sc.nextInt();
            if(isPrime(n)){
                cnt++;
            }
        }
        System.out.println(cnt);

    }
    public static boolean isPrime(int num){
        if(num < 2){
            return false;
        }

        for (int i = 2; i < Math.sqrt(num); i++) {
            if(num % i == 0){
                return false;
            }
        }
        return true;
    }
}
