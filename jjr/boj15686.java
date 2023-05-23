import java.util.ArrayList;
import java.util.Scanner;

public class boj15686 {
    public static int[][] arr;
    public static ArrayList<Node> houseList = new ArrayList<>();;
    public static ArrayList<Node> chickenMarketList = new ArrayList<>();;
    public static int answer = Integer.MAX_VALUE;

    public static int m;

    public static boolean[] visit ;

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        m = sc.nextInt();

        arr = new int[n][n];


        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                arr[i][j] = sc.nextInt();
                if(arr[i][j] == 1){
                    houseList.add(new Node(i, j));
                }
                else if(arr[i][j] == 2){
                    chickenMarketList.add(new Node(i, j));
                }
            }
        }
        visit = new boolean[chickenMarketList.size()];
        chickenMarketCheck(0, 0);

        System.out.println(answer);


        // 로직
        // 치킨거리는 집과 치킨집의 거리차이
        // 도시의 치킨거리가 가장 작게되는 경우의 값을 구한다.
        // 도시의 치킨거리가 가장 작게되는 경우는 모든 치킨집의 조합을 (치킨집 = c) cCm 했을때 모든 집에서 치킨거리를 다 더한값이 가장 작은 경우이다.
        // 시간제한 1초
        // N> 50, m>13
        //
    }

    public static void chickenMarketCheck(int depth, int count){
        if(count == m){
            minCheck();
            return;
        }


        for (int i = depth; i < chickenMarketList.size(); i++) {
            if(!visit[i]){
                visit[i] = true;
                chickenMarketCheck(i+1, count+1);
                visit[i] = false;
            }

        }

    }

    // 최솟값 체크
    public static void minCheck(){
        // 최소거리의 합 구할 변수
        int sum = 0;
        // 집 돌면서 현재 선택된 치킨집과의 치킨거리 구해서 값 더하기
        for (Node houseNode : houseList) {
            // 현재집의 치킨거리중 가장 가까운 치킨거리 구하기
            int chickenLen = Integer.MAX_VALUE;
            for (int i = 0; i < chickenMarketList.size(); i++) {
                if(visit[i]){
                    Node chickenMarketNode = chickenMarketList.get(i);
                    int value = Math.abs(houseNode.getX() - chickenMarketNode.getX()) + Math.abs(houseNode.getY() - chickenMarketNode.getY());
                    chickenLen = Math.min(chickenLen, value);
                }
            }
            // 집마다 가장 가까운 치킨거리 더해주기
            sum+=chickenLen;
        }
        // 현재 설정된 치킨거리보다 작으면 변경
        answer = Math.min(answer, sum);

    }

    public static class Node{
        private int x;
        private int y;

        Node(int x, int y){
            this.x = x;
            this.y = y;

        }

        public int getX() {
            return x;
        }
        public int getY() {
            return y;
        }


    }

}
