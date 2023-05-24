import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.StringTokenizer;

public class boj2580 {
    static int[][] map;
    static ArrayList<Node> emptyList = new ArrayList<>();
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        map = new int[9][9];

        for (int i = 0; i < 9; i++) {
            StringTokenizer st = new StringTokenizer(br.readLine());
            for (int j = 0; j < 9; j++) {
                map[i][j] = Integer.parseInt(st.nextToken());
                if(map[i][j] == 0){
                    emptyList.add(new Node(i, j, false));
                }
            }
        }


        sudoku(0);


    }

    public static void sudoku( int index){
        if(index == emptyList.size()){
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < map.length; i++) {
                for (int j = 0; j < map.length; j++) {
                    sb.append(map[i][j]).append(" ");
                }
                sb.append("\n");
            }
            System.out.println(sb);
            System.exit(0);
        }
        Node node =  emptyList.get(index);
        for (int i = 1; i <= 9; i++) {
            if(validCheck(node.x, node.y, i)){
                map[node.x][node.y] = i;
                sudoku(index + 1);
            }
            if(i == 9){
                map[node.x][node.y] = 0;
            }

        }


    }

    // node 객체
    public static class  Node{
        int x;
        int y;


        public Node(int x, int y, boolean visit) {
            this.x = x;
            this.y = y;
        }
    }

    // 가로세로네모에 겹치는 수가 없으면 true
    public static boolean validCheck(int x, int y, int value){
        if(colCheck(x, value) && rowCheck(y, value) && squareCheck(x, y, value) ){
            return true;
        }
        return false;
    }

    public static boolean colCheck(int x,  int value){
        for (int i = 0; i < 9; i++) {
            if(map[x][i] == value){
                return false;
            }
        }
        return true;
    }
    public static boolean rowCheck(int y,  int value){
        for (int i = 0; i < 9; i++) {
            if(map[i][y] == value){
                return false;
            }
        }
        return true;
    }
    public static boolean squareCheck(int x, int y,  int value){
        int squarePositionX = (x/3) * 3 ;
        int squarePositionY = (y/3) * 3;

        for (int i = squarePositionX; i < squarePositionX+3; i++) {
            for (int j = squarePositionY; j < squarePositionY+3; j++) {
                if(map[i][j] == value) return false;
            }
        }
        return true;
    }


}
