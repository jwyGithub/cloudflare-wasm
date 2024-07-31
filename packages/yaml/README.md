# cloudflare-wasm

Using Wasm in CloudFlare

## Install

### with pnpm

```bash
pnpm add @jiangweiye/cloudflare-wasm-yaml
```

### with npm

```bash
npm install @jiangweiye/cloudflare-wasm-yaml
```

### with yarn

```bash
yarn add @jiangweiye/cloudflare-wasm-yaml
```

## Use

```typescript
import init, { dump, load } from '@jiangweiye/cloudflare-wasm-yaml';

export default {
    async fetch(request: Request, env: Env): Promise<Response> {
        await init();

        const data = load('name: Cloudflare Workers\n');
        data.set('name', 'Cloudflare Workers');

        const output = dump(data);

        return new Response(output, {
            headers: { 'content-type': 'text/plain' }
        });
    }
};
```
