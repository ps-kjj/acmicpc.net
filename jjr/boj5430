import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayDeque;
import java.util.Deque;
import java.util.StringTokenizer;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringTokenizer st;
        StringBuilder sb = new StringBuilder();
        int t = Integer.parseInt(br.readLine());

        for (int i = 0; i < t; i++) {
            String str = br.readLine();
            int n = Integer.parseInt(br.readLine());
            st = new StringTokenizer(br.readLine(), "[],");
            Deque<Integer> deque = new ArrayDeque<>();

            for (int j = 0; j < n; j++) {
                deque.add(Integer.parseInt(st.nextToken()));
            }

            boolean head = true;
            boolean isError = false;
            for (int j = 0; j < str.length(); j++) {
                switch(str.charAt(j)){
                    case 'R':
                        head = !head;
                        continue;
                    case 'D':
                        if(head){
                            if(deque.pollFirst() == null){
                               isError = true;
                            }
                        }else{
                            if(deque.pollLast() == null){
                                isError = true;
                            }
                        }
                        break;
                }
            }
            if(isError){
                sb.append("error\n");
                continue;
            }
            sb.append("[");
            if (head){
                if(!deque.isEmpty()){
                    sb.append(deque.pollFirst());
                }
                while(!deque.isEmpty()){
                    sb.append(",").append(deque.pollFirst());
                }
            }else{
                if(!deque.isEmpty()){
                    sb.append(deque.pollLast());
                }
                while(!deque.isEmpty()){
                    sb.append(",").append(deque.pollLast());
                }
            }
            sb.append("]\n");
        }
        System.out.println(sb);
    }
}
