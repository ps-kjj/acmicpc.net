import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringBuilder sb = new StringBuilder();
        int n = Integer.parseInt(br.readLine());
        for (int i = 0; i < n; i++) {
            StringTokenizer st = new StringTokenizer(br.readLine());
            int m = Integer.parseInt(st.nextToken());
            int num = Integer.parseInt(st.nextToken());
            Queue<Pair<Integer, Boolean>> queue = new LinkedList<>();
            int[] arr = new int[m];
            st = new StringTokenizer(br.readLine());;
            for (int j = 0; j < m; j++) {
                int qNum = Integer.parseInt(st.nextToken());
                arr[j] = qNum;
               if(j == num){
                   queue.offer(new Pair(qNum, true));
               }else{
                   queue.offer(new Pair(qNum, false));
               }
            }
            Integer[] numInt = Arrays.stream(arr).boxed().toArray(Integer[]::new);
            Arrays.sort(numInt, Collections.reverseOrder());
            int cnt  = 0;
            int index = 0;
            while(!queue.isEmpty()){
                if(queue.peek().getValue1() != numInt[index]){
                    queue.offer(queue.poll());
                }else{
                    cnt++;
                    index++;
                    if(queue.poll().getValue2()){
                        break;
                    }
                }
            }
            sb.append(cnt).append("\n");
        }

        System.out.println(sb);
    }
    public static class Pair<T1, T2>{
        private T1 value1;
        private T2 value2;

        Pair(){};
        Pair(T1 value1, T2 value2){
            this.value1 = value1;
            this.value2 = value2;
        }

        public void setValue1(T1 value1) {
            this.value1 = value1;
        }

        public void setValue2(T2 value2) {
            this.value2 = value2;
        }

        public T1 getValue1() {
            return value1;
        }
        public T2 getValue2(){
            return value2;
        }
    }

}
