import java.util.Scanner;

public class Main {
    public static int[][]  arr = null;
    public static boolean[] visit = null;
    public static int teamSize = 0;
    public static int n;
    public static int min = Integer.MAX_VALUE;

    public static void main(String[] args) {
        /**
         * 두개의 팀으로 나누고 두개의 팀의 능력치의 차이가 가장 적으면 해당 능력치 차이를 출력한다.
         * 전체 팀의 크기는 무조건 2로 나누어진다.
         */
        Scanner sc = new Scanner(System.in);
        n = sc.nextInt();

        teamSize = n/2;
        arr = new int[n][n];
        visit = new boolean[n];
        // 초기배열 입력
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                arr[i][j] = sc.nextInt();
            }
        }
        // 모든 조합의 힘차이를 검색해봐야함
        combination(0, 0);

        System.out.println(min);

    }

    public static void combination(int diff, int cnt){
        if(cnt == teamSize){
            // 두개의 값 검사
            min = powerCheck();
            return;
        }
        for (int i = diff; i < n; i++) {
            if(!visit[i]){
                visit[i] = true;
                combination(i+1, cnt+1);
                visit[i] = false;
            }
            
        }

    }

    public static int powerCheck(){
        int startTeam = 0;
        int linkTeam = 0;
        for (int i = 0; i < n-1; i++) {
            for (int j = i + 1; j < n; j++) {
                if(visit[i] && visit[j]){
                    startTeam += arr[i][j];
                    startTeam += arr[j][i];
                } else if(!visit[i] && !visit[j]){
                    linkTeam += arr[i][j];
                    linkTeam += arr[j][i];
                }
            }
        }


        int power = Math.abs(startTeam - linkTeam);

        if(power == 0){
            return 0;
        }

        return Math.min(power, min);
    }

}
