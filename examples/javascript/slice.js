const fs = require('fs')
const path = require('path')

const f1 = fs.readFileSync(path.resolve(__dirname, './file1.ts')).toString()
const f2 = fs.readFileSync(path.resolve(__dirname, './file2.ts')).toString()

console.warn("f1")
console.log(f1.slice(0, 2074))
console.warn("f2")
console.log(f2.slice(2597, 4701))