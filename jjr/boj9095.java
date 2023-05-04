import java.util.Scanner;

/**
 * 1, 2, 3 더하기
 * 시간제한 1초 - 1억연산
 * 입력 : 정수 N - 양수 n < 11
 * 정수 4를 1, 2, 3의 합으로 나타내는 방법은 총 7가지가 있다. 합을 나타낼 때는 수를 1개 이상 사용해야 한다.
 *
 * 고정적인 값 1, 2, 3을 사용 (가장 큰 값이 6)
 * 규칙 찾기
 * 1일 경우의 수 1
 * 1
 * 2일 경우의 수 2
 * 1 + 1
 * 2
 * 3일 경우의 수 4
 * 1 + 1 + 1
 * 1 + 2
 * 2 + 1
 * 3
 * 4일 경우의 수 7
 * 1 + 1 + 1 + 1
 * 1 + 1 + 2
 * 1 + 2 + 1
 * 2 + 1 + 1
 * 2 + 2
 * 1 + 3
 * 3 + 1
 * 반복적인 경우의 수
 * n = (n-1) + (n-2) + (n-3)
 *
 */

public class boj9095 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int t = sc.nextInt();
        /**
         * dp[0] = 0;
         * dp[1] = 1;
         * dp[2] = 2;
         * dp[3] = 3;
         * dp[4] = 7;
         */
        int[] arr = new int[11];
        arr[1] = 1;
        arr[2] = 2;
        arr[3] = 4;
        for (int i = 4; i < arr.length; i++) {
            arr[i] = arr[i-1] + arr[i-2] + arr[i-3];
        }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < t; i++) {
            int n = sc.nextInt();
            sb.append(arr[n]).append("\n");
        }
        System.out.println(sb);

    }

}
