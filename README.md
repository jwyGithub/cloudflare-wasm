# cloudflare-wasm

Using Wasm in CloudFlare

## Packages

-   [@jiangweiye/cloudflare-wasm-yaml](#wasm-yaml)
-   [@jiangweiye/cloudflare-wasm-image](#wasm-image)

### wasm-yaml

使用 wasm 解析 yaml 文件，以及将 yaml 文件转换为 map 对象。

[使用示例](https://github.com/jwyGithub/cloudflare-wasm/tree/main/packages/yaml)

```typescript
import { dump, load } from '@jiangweiye/cloudflare-wasm-yaml';

export default {
    async fetch(request: Request, env: Env): Promise<Response> {
        const data = load('name: Cloudflare Workers\n');
        data.set('name', 'Cloudflare Workers');

        const output = dump(data);

        return new Response(output, {
            headers: { 'content-type': 'text/plain' }
        });
    }
};
```

### wasm-image

使用 wasm 处理图片，支持图片转换，压缩， 等。

[使用示例](https://github.com/jwyGithub/cloudflare-wasm/tree/main/packages/image)
