import java.util.*;
import java.io.*;

public class GridSearch {
//public class Solution {

    public static int DB_WIDTH;
    public static int DB_HEIGHT;

    public static int SEARCH_WIDTH;
    public static int SEARCH_HEIGHT;

    public static String searchInArray(int[][] db, int[][] search) {
        int top_corner = search[0][0];
        int search_y = 0;
        int search_x = 0;
        for (int i = 0; i < DB_HEIGHT; i++) {
            for (int j = 0; j <  DB_WIDTH; j++) {
                System.out.println("DB y: " + i + " x: " + j);
                System.out.println("Search y: " + search_y + " search x: " + search_x);
                if (db[i][j] == search[search_y][search_x]) {
                    //for (int search_i = 0; search_i < SEARCH_HEIGHT; search_i++) {
                    //    for (int search_j = 0; search_j < SEARCH_WIDTH; search_j++) {
                    //        if ( search[search_i][search_j] != db[i][j] ) {
                    //            return "NO";
                    //        }
                    //    }
                    //}
                    int DB_pos_x = j;
                    int DB_pos_y = i;
                    for (int somei = 0; somei < SEARCH_HEIGHT; somei++) {
                        for (int somej = 0; somej < SEARCH_WIDTH; somej++) {
                            if (search[somei][somej] == db[DB_pos_y][DB_pos_x]) {
                                if (DB_pos_x < SEARCH_WIDTH) {
                                    DB_pos_x++;
                                }
                            }
                        }
                        if (DB_pos_y < SEARCH_HEIGHT) {
                            DB_pos_y++;
                        } else if (DB_pos_y == SEARCH_HEIGHT) {
                            return "YES";
                        }
                    }
                    //System.out.println("Correct so far");
                    //if (search_x < SEARCH_WIDTH)
                    //    search_x++;
                    //else if (search_x == SEARCH_WIDTH) {
                    //    search_y++;
                    //}
                }
            }
            //if (search_y < SEARCH_HEIGHT) {
            //    search_y++;
            //}
        }
        return "NO";
    }

    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int questions = Integer.parseInt(in.nextLine());

        for (int i = 0; i < questions; i++) {

            // Read database array size (x, y)
            String db_line = in.nextLine();
            String[] db_size_str = db_line.split(" ");
            DB_WIDTH = Integer.parseInt(db_size_str[1]);
            DB_HEIGHT = Integer.parseInt(db_size_str[0]);
            int[] db_size = { DB_HEIGHT, DB_WIDTH };

            // Read y lines of x chars
            int[][] db = new int[db_size[0]][db_size[1]];
            for (int j = 0; j < DB_HEIGHT; j++) {
                String line = in.nextLine();
                for (int k = 0; k < DB_WIDTH; k++) {
                    db[j][k] = Character.getNumericValue(line.charAt(k));
                }
            }

            // 2D array printer
            //for (int[] item : db) {
            //    System.out.println(Arrays.toString(item));
            //}

            // Read tab seperated size of search array (x, y)
            String search_line = in.nextLine();
            String[] search_size_str = search_line.split("\t");
            SEARCH_WIDTH = Integer.parseInt(search_size_str[1]);
            SEARCH_HEIGHT = Integer.parseInt(search_size_str[0]);
            int[] search_size = { SEARCH_HEIGHT, SEARCH_WIDTH };

            // Read y lines of x chars
            int[][] search = new int[search_size[0]][search_size[1]];
            for (int x = 0; x < SEARCH_HEIGHT; x++) {
                String line = in.nextLine();
                for (int y = 0; y < SEARCH_WIDTH; y++) {
                    search[x][y] = Character.getNumericValue(line.charAt(y));
                }
            }

            // Print 'YES' or 'NO'
            //for (int[] item : search) {
            //    System.out.println(Arrays.toString(item));
            //}

            System.out.println(searchInArray(db, search));
        }
    }
}
