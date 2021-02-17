const fs = require('fs')
const path = require('path')

const f1 = fs.readFileSync("/Users/jiangwei/projects/ruaaa/the-first-code-of-rust/examples/jsx/file1.tsx").toString()
const f2 = fs.readFileSync("/Users/jiangwei/projects/ruaaa/the-first-code-of-rust/examples/jsx/file2.tsx").toString()

console.warn("f1")
console.log(f1.length, f2.length)
// console.log(f1.slice(217, 273))
console.warn("f2")
// console.log(f2.slice(200, 207))