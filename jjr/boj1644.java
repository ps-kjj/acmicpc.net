import java.util.*;

public class boj1644 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();

        ArrayList<Integer> list = new ArrayList<>();


        for (int i = 2; i <= n; i++) {
            if(isPrime(i)){
                list.add(i);
            }
        }


        int answer = 0;
        int size = list.size();

        int left =0, right =0;
        int sum = 0;
        while(true) {
            if(sum == n){
                answer++;
                sum -= list.get(left++);
            }else if(sum > n){
                sum -= list.get(left++);
            } else if(right == size){
               break;
            } else{
                sum += list.get(right++);
            }


        }

        System.out.println(answer);

    }

    public static boolean isPrime(int num) {
        for (int j = 2; j*j <= num; j++) {
             if(num % j == 0) return false;
        }

        return true;
    }
}
