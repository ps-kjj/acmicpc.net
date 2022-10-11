import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class baekjoon10989 {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringBuilder sb = new StringBuilder();
        // 카운팅을 위한 배열 선언
        int[] list = new int[10001];
        // n 읽기
        int n = Integer.parseInt(br.readLine());

        // n개의 수 읽어서 list에 넣기
        for (int i = 0; i < n; i++) {
            list[Integer.parseInt(br.readLine())]++;
        }
        br.close();

        // 문자열 생성
        for (int i = 0; i < list.length; i++) {
            while(list[i] > 0){
                sb.append(i).append("\n");
                list[i]--;
            }
        }

        // 출력
        System.out.println(sb);
    }
}
