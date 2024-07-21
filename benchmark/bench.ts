import { Bench } from 'tinybench'

const b = new Bench()

b.add('Native a + 100', () => {

})

b.add('JavaScript a + 100', () => {

})

await b.run()

console.table(b.table())
