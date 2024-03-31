import "./style.css";

let input = document.querySelector("#input");
let isPalindrome = document.querySelector("#is-palindrome");

let inputArray;

input.addEventListener("keyup", () => {
  console.log(input.value.trim(" "));
  inputArray = input.value.split("");
  let normalString = input.value.toLowerCase();
  let reversedString = scanBackwards(inputArray);
  if (normalString.length === 1 || normalString.length === 0) {
    isPalindrome.textContent = "";
  } else if (normalString === reversedString) {
    isPalindrome.textContent = "true";
  } else {
    isPalindrome.textContent = "false";
  }
});

function scanBackwards(val) {
  let backArray = [];
  val.forEach((current) => {
    backArray.unshift(current);
  });
  return backArray.join("").toLowerCase();
}
