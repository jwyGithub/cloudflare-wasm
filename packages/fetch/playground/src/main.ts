import init, { load, dump } from '@jiangweiye/cloudflare-wasm-yaml';
import source from './source.yml?raw';

(async () => {
    await init();

    const data = load(source);
    data.set('name', 'Cloudflare Workers');

    const output = dump(data);

    console.log('output', output);
})();
