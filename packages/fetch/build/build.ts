import path from 'node:path';
import process from 'node:process';
import { execa } from 'execa';

const __dirname = path.dirname(new URL(import.meta.url).pathname);
const wasm_exec_dir = path.resolve(__dirname, '../core');

async function build() {
    await execa('wasm-pack', ['build', '--target', 'web', '--out-dir', '../dist'], {
        cwd: wasm_exec_dir,
        stdio: 'inherit'
    })
        .catch(err => {
            console.error(err);
            process.exit(1);
        })
        .finally(() => {
            // 删除输出目录下的package.json文件 .gitignore
            execa('rm', ['-rf', path.resolve(__dirname, '../dist/package.json')]);
            execa('rm', ['-rf', path.resolve(__dirname, '../dist/.gitignore')]);
        });
}

build();
