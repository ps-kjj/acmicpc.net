import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Stack;

public class Main {
    public static void main(String[] args) throws IOException {
        Stack<Integer> stack = new Stack<>();
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringBuilder sb = new StringBuilder();
        int n = Integer.parseInt(br.readLine());

        int point = 0;
        
        while(n-- > 0){
            int num = Integer.parseInt(br.readLine());
            if( num > point){
                for (int i = point+1; i <= num; i++) {
                    stack.push(i);
                    sb.append("+").append("\n");
                }
                point=num;
            } else if(stack.peek() != num){
                System.out.println("NO");
                return;
            }
            stack.pop();
            sb.append("-").append("\n");
        }

        System.out.println(sb);

    }
}
