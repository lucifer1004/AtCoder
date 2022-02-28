import 'dart:io';

void main() {
  var params = read_ints();
  var n = params[0];
  var x = params[1];
  var can = List<bool>.filled(x + 1, false);
  can[0] = true;
  for (var i = 0; i < n; i++) {
    var params = read_ints();
    var a = params[0];
    var b = params[1];
    var nxt = List<bool>.filled(x + 1, false);
    for (var j = 0; j < x; j++) {
      if (can[j]) {
        if (j + a <= x) {
          nxt[j + a] = true;
        }
        if (j + b <= x) {
          nxt[j + b] = true;
        }
      }
    }
    can = nxt;
  }

  if (can[x]) {
    print('Yes');
  } else {
    print('No');
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
