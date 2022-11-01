import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.HashSet;
import java.util.StringTokenizer;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        HashSet<Integer> set = new HashSet<>();

        int n = Integer.parseInt(br.readLine());
        StringTokenizer st = new StringTokenizer(br.readLine());
        for (int i = 0; i < n; i++) {
            set.add(Integer.parseInt(st.nextToken()));
        }

        Integer[] arr = set.toArray(new Integer[set.size()]);
        Arrays.sort(arr);
        for (Integer i : arr) {
            System.out.print(i + " ");
        }
    }
}
