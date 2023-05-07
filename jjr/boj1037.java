import java.util.Scanner;

public class boj1037 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();

        int firstN = sc.nextInt();
        int maxNum = firstN;
        int minNum = firstN;
        for (int i = 1; i < n; i++) {
            int checkNum = sc.nextInt();
            if(checkNum > maxNum){
                maxNum = checkNum;
            }
            if(checkNum < minNum){
                minNum = checkNum;
            }
        }
        System.out.println(maxNum*minNum);

    }
}
