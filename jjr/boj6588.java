import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class boj6588 {
    public static void main(String[] args) throws IOException {

        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));



        // 소수 미리 배열로 만들기
        boolean[] arr = new boolean[1000001];

        arr[0] = arr[1] = true;

        for (int i = 2; i < arr.length; i++) {
            arr[i] = isPrime(i);
        }
        StringBuilder sb = new StringBuilder();
        int n = Integer.parseInt(br.readLine());
        while(n != 0){
            boolean isEnd = true;
            for (int i = 2; i <= n/2; i++) {
                // 현재 숫자 + 현재숫자 뺀값 두 수는 모두 소수여야함
                if(arr[i] && arr[n - i]){
                    isEnd = false;
                    sb.append(n + " = " + i + " + " + (n-i));
                    break;
                }
            }
            if(isEnd){
                sb.append("Goldbach's conjecture is wrong.");
            }
            sb.append("\n");
            n = Integer.parseInt(br.readLine());
        }
        System.out.println(sb);



    }
    public static boolean isPrime(int num) {
        for (int j = 2; j*j <= num; j++) {
            if(num % j == 0) return false;
        }
        return true;
    }
}
