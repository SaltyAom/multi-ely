import { $ } from 'bun'
import { cpus } from 'os'

const process = <Promise<any>[]>[]

let cpu = cpus().length
const model = cpus()[0].model

if(model.includes("Intel(R)")) {
    const gen = +model.split(" ")[0].replace("th", "")

    if(!Number.isNaN(gen) && gen >= 12) {
        // Intel 12th gen are using P/E Core
        // request only P core which is half core in most CPU
        cpu = cpu / 2
    } 
}

$.cwd(import.meta.dir)
for(let i = 0; i < cpu; i++)
    process.push($`NODE_ENV=production bun src/index.tsx`)

await Promise.all(process)
