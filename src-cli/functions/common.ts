import path from 'path';
import fs from 'fs';

export function isRepogitoryRoot() {
  const termianlPath = process.cwd();
  return isExistFile(path.join(termianlPath, 'package.json'));
}

export function isExistFile(path: string) {
  return fs.existsSync(path);
}