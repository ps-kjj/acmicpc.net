import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class boj2661 {
    static int n;
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        n = Integer.parseInt(br.readLine());

        // 현재의 번호가 부분수열이 될려면 최소한 앞의 개이상의 수와 현재 값을 포함하는 수열이 같은지 확인해야함 최소 2번부터 확인이 가능
        String str = "";

        backTracking(str);

    }

    public static void backTracking(String str){
    
        if(str.length() == n){
            System.out.println(str);
            // 프로세스 강제종료
            System.exit(0);
        }


        for (int i = 1; i <= 3; i++) {
           if(isSeq(str + i)){
               backTracking(str + i);
           }
        }

    }

    public static boolean isSeq(String str){
        int len = str.length();
        // len = 3
        // 1, 2, 3,  1, 2
        for (int i = 1; i <= len/2; i++) {
            // 문자열 비교
            if(str.substring(str.length() -i).equals(
                    str.substring(str.length()-2*i, str.length() - i)) ){
                return false;
            }
        }
        return true;
    }
}
