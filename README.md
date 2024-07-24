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

## example

```typescript
import init, { dump, load } from '@jiangweiye/cloudflare-wasm-yaml';
import source from './source.yml?raw';

(async () => {
    await init();

    const data = load(source);
    data.set('name', 'Cloudflare Workers');

    const output = dump(data);

    console.log('output', output);
})();
```
