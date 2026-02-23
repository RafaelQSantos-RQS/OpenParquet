import fs from 'fs';
import { execSync } from 'child_process';
import process from 'process';

const newVersion = process.env.npm_package_version;

if (!newVersion) {
    console.error("Erro: Este script deve ser rodado pelo 'npm version'");
    process.exit(1);
}

console.log(`🔄 Sincronizando arquivos para a versão ${newVersion}...`);

const tauriPath = 'src-tauri/tauri.conf.json';
const tauriConf = JSON.parse(fs.readFileSync(tauriPath, 'utf8'));
tauriConf.version = newVersion;
fs.writeFileSync(tauriPath, JSON.stringify(tauriConf, null, 2));
console.log(`✅ ${tauriPath} atualizado.`);

const constantsPath = 'src/lib/constants.ts';
let constantsContent = fs.readFileSync(constantsPath, 'utf8');
const versionRegex = /version:\s*'v[^']*'/;
const newVersionLine = `version: 'v${newVersion}'`;
if (versionRegex.test(constantsContent)) {
    constantsContent = constantsContent.replace(versionRegex, newVersionLine);
    fs.writeFileSync(constantsPath, constantsContent);
    console.log(`✅ ${constantsPath} atualizado.`);
} else {
    console.warn(`⚠️ Não foi possível encontrar a versão no constants.ts.`);
}

const xmlPath = 'src-tauri/linux/com.rafaelqsantos.openparquet.metainfo.xml';
let xmlContent = fs.readFileSync(xmlPath, 'utf8');

// Data de hoje no formato YYYY-MM-DD
const today = new Date().toISOString().split('T')[0];

const releaseRegex = /<release version="[^"]*" date="[^"]*" \/>/;
const newReleaseTag = `<release version="${newVersion}" date="${today}" />`;

if (releaseRegex.test(xmlContent)) {
    xmlContent = xmlContent.replace(releaseRegex, newReleaseTag);
    fs.writeFileSync(xmlPath, xmlContent);
    console.log(`✅ ${xmlPath} atualizado.`);
} else {
    console.warn(`⚠️ Não foi possível encontrar a tag <release> no XML para atualizar.`);
}

try {
    execSync(`git add ${tauriPath} ${constantsPath} ${xmlPath}`);
    console.log(`✅ Arquivos adicionados ao git.`);
} catch (e) {
    console.error("Erro ao adicionar arquivos ao git:", e);
    process.exit(1);
}