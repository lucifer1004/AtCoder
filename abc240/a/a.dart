import 'dart:io';

void main() {
  var params = read_ints();
  var delta = (params[0] - params[1]).abs();
  if (delta == 1 || delta == 9) {
    print("Yes");
  } else {
    print("No");
  }
}

List<int> read_ints() {
  var inputText = stdin.readLineSync();
  if (inputText != null) {
    return inputText.trim().split(' ').map((e) => int.parse(e)).toList();
  } else {
    return [];
  }
}
