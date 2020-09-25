import * as wasm from 'crank';

// wasm.greet('visitor');

document.getElementById('text').addEventListener('input', function (event) {
  var result = wasm.parse_formula(event.target.value);
  if (result) {
    document.getElementById('result').innerHTML = 'VALID';
  } else {
    document.getElementById('result').innerHTML = 'INVALID';
  }
})
