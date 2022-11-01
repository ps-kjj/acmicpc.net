import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.HashMap;
import java.util.HashSet;
import java.util.StringTokenizer;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        HashMap<Integer, Integer> map = new HashMap<>();

        int n = Integer.parseInt(br.readLine());

        StringTokenizer st = new StringTokenizer(br.readLine());
        for (int i = 0; i < n; i++) {
            int nt = Integer.parseInt(st.nextToken());
            if(map.containsKey(nt)){
                map.put(nt, map.get(nt) + 1);
            }else{
                map.put(nt, 1);
            }
        }
        int m  = Integer.parseInt(br.readLine());
        st = new StringTokenizer(br.readLine());
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < m; i++) {
            int nt = Integer.parseInt(st.nextToken());
            if(map.containsKey(nt)){
                sb.append(map.get(nt)).append(" ");
            }else{
                sb.append(0).append(" ");
            }
        }
        System.out.println(sb);
    }
}
