import java.util.Scanner;

public class boj9663 {

    //검사할 체크판 로우값 저장 cols는 depth(배열의 크기)로 대체
    public static int[] chessPan;
    // 입력값
    public static int N;
    // 반환값
    public static int cnt=0;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        N = sc.nextInt();

        chessPan = new int[N];


        // 체크판의 위에서부터 n*n 크기에 n개의 말들이 들어간다는건 하나의 행에 하나의 말이 들어가야한다는 뜻으로 한줄씩 검사하면 될듯
        // 체크판을 재귀로 검사 (위아래 대각선 체크해서 있으면 넘어가기)
        // 모든 검사 통과했을때 n번만큼 무사히 반복하며 cnt 증가
        queenCheck(0);

        System.out.println(cnt);

    }

    public static void queenCheck(int depth){
        // N만큼 왔으면 정지
        if(N == depth){
            cnt++;
            return;
        }

        // 깊이마다 검색
        for (int i = 0; i < N; i++) {
            chessPan[depth] = i;
            if(checkPosition(depth)){
                queenCheck(depth + 1);
            }
        }
    }

    //row 검색(현재 깊이)
    public static boolean checkPosition(int cols){
        //이전 깊이까지 검사
        for (int i = 0; i < cols; i++) {
            // 현재 깊이의 row 값 검사
            if(chessPan[i] == chessPan[cols]){
               return false;
            }
            // 대각선 검사
            // 대각선은 현재가 만약 깊이가 3이라고 치면 2의 깊이에 있는 값은 현재 row 와의 차이가 -1 이거나 +1 이어야함
            // 현재 3의 깊이의 2번 로우를 찾아보고 있다면 2의 깊이의 값의 +- 1 의 위치에 값이 있으면 안되니 1 or 3이면 안됨
            // 해당 값을 비교하려면 |2 - (찾아보려는 깊이의 값)|이 1이면 대각선의 위치에 값이 있는 것
            // 코드로 구현하면 |현재 깊이 - 찾아보려고하는 깊이| == |현재 깊이의 값 - 찾아보려고하는 깊이의 값|
           else if(Math.abs(cols-i) == Math.abs(chessPan[cols] - chessPan[i]) ){
               return false;
           }
        }
        return true;
    }

}
