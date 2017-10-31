function fibonacci(n) {
  if (n === 0) {return 0;}
  if (n === 1) {return 1;}
  
  return fibonacci(n - 1) + fibonacci(n - 2);
}

for (let i = 2; i < 12; i++) {
  console.log(fibonacci(i));
}
