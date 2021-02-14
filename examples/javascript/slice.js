const fs = require('fs')
const path = require('path')

const f1 = fs.readFileSync("/Users/jiangwei/projects/ruaaa/jscpd/build-utils/prismjs-languages-concat.ts").toString()
const f2 = fs.readFileSync("/Users/jiangwei/projects/ruaaa/jscpd/examples/server/src/main.ts").toString()

console.warn("f1")
console.log(f1.slice(217, 273))
console.warn("f2")
console.log(f2.slice(200, 207))