import { $ } from 'bun'
import { join } from 'path'

console.log("Elysia")
let process = Bun.spawn({
    cmd: ['bun', 'elysia/spawn.ts'],
    env: {
        ...Bun.env,
        NODE_ENV: 'production'
    }
})
await Bun.sleep(1000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
await Bun.sleep(500)

console.log("Axum")
process = Bun.spawn({
    cmd: ['cargo', 'run', '--release'],
    env: {
        NODE_ENV: 'production'
    },
    cwd: join(import.meta.dirname, 'axum')
})
await Bun.sleep(2000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
await Bun.sleep(500)

console.log("Actix")
process = Bun.spawn({
    cmd: ['cargo', 'run', '--release'],
    env: {
        NODE_ENV: 'production'
    },
    cwd: join(import.meta.dirname, 'axum')
})
await Bun.sleep(2000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
