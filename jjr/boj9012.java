import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Stack;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        int n = Integer.parseInt(br.readLine());
        StringBuilder sb = new StringBuilder();

        for (int i = 0; i < n; i++) {
            Stack<Character> stack = new Stack<>();
            boolean isVPS = true;
            String str = br.readLine();
            for (int j = 0; j < str.length(); j++) {
                if(str.charAt(j) == '('){
                    stack.push(str.charAt(j));
                }else{
                    if(!stack.isEmpty()){
                        stack.pop();
                    }else{
                        isVPS = false;
                        break;
                    }
                }
            }
            if(!stack.isEmpty()){
                isVPS = false;
            }
            if(isVPS){
                sb.append("YES").append(" ");
            }else{
                sb.append("NO").append(" ");
            }

        }
        System.out.println(sb);
    }
}

