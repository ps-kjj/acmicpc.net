import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.HashSet;
import java.util.StringTokenizer;

public class boj11650 {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        HashSet<Num> set = new HashSet<>();

        int n = Integer.parseInt(br.readLine());
        for (int i = 0; i < n; i++) {
            StringTokenizer st = new StringTokenizer(br.readLine());
            set.add(new Num(Integer.parseInt(st.nextToken()), Integer.parseInt(st.nextToken())));
        }
        Num[] nums = set.toArray(new Num[set.size()]);
        Arrays.sort(nums, (o1, o2) -> {
            if(o1.x == o2.x){
                return o1.y - o2.y;
            }else{
                return o1.x - o2.x;
            }
        });
        for (Num num : nums) {
            System.out.println(num.x + " " + num.y);
        }

    }
    static class Num {
        public int x;
        public int y;
        Num(int x, int y){
            this.x = x;
            this.y = y;
        }

    }

}
