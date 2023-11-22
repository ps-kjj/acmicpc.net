import java.util.Scanner;

public class boj2485 {
    public static int[] arr ;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);


        int n = sc.nextInt();

        int[] arr = new int[n];
        int[] gap = new int[n-1];



        for (int i = 0; i < n ; i++) {
            arr[i] = sc.nextInt();
            if(i > 0){
                gap[i-1] = arr[i] - arr[i-1];
            }
        }

        for(int i=1; i< gap.length; i++){
            gap[i] = gcd(gap[i-1], gap[i]);
        }

        System.out.println((arr[n-1] - arr[0])/gap[gap.length -1] + 1 - n );



    }

    public static int gcd(int x, int y){
        if(y == 0){
            return x;
        } else {
            return gcd(y, x%y);
        }

    }
}
