import java.io.IOException;
import java.util.Scanner;

public class Main {
    public static boolean[] check;
    public static int[] numList;
    public static StringBuilder sb;
    public static int n;
    public static void main(String[] args) throws IOException {
        Scanner sc = new Scanner(System.in);
        sb = new StringBuilder();
        while(true){
            n = sc.nextInt();
            if(n == 0){
                break;
            }
            check = new boolean[n];
            numList = new int[n];
            for (int i = 0; i < n; i++) {
                numList[i] = sc.nextInt();
            }
            dfs(0, 0);
            sb.append("\n");
        }
        System.out.println(sb);
    }
    private static void dfs(int start, int depth){
        if(depth == 6){
            for (int i = 0; i < n; i++) {
                if(check[i]){
                    sb.append(numList[i] + " ");
                }
            }
            sb.append("\n");
        }

        for (int i = start; i < n; i++) {
            check[i] = true;
            dfs(i+1, depth+1);
            check[i] =false;
        }
    }
}
