import java.nio.file.*;
import java.nio.charset.StandardCharsets;
import java.io.*;

class Day1 {
	public static void main(String[] args) throws IOException {
		Path p = FileSystems.getDefault().getPath("inputs/day1.txt");
		BufferedReader reader = Files.newBufferedReader(p, StandardCharsets.US_ASCII);

		String line = "";
		int floor = 0;

		while ((line = reader.readLine()) != null) {
			for (int i = 0; i < line.length(); i++) {
				if (line.charAt(i) == '(') {
					floor++;
				} else if (line.charAt(i) == ')') {
					floor--;
				}
				if (floor == -1) {
					System.out.println(i + 1);
					break;
				}
			}
		}
	}
}

