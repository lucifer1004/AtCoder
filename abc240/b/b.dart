import 'dart:io';
import 'dart:collection';

void main() {
  read_ints();
  var a = read_ints();
  var s = HashSet.from(a);
  print(s.length);
}

List<int> read_ints() {
  var inputText = stdin.readLineSync();
  if (inputText != null) {
    return inputText.trim().split(' ').map((e) => int.parse(e)).toList();
  } else {
    return [];
  }
}
