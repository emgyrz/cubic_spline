#!/usr/bin/env node
const fs = require('fs')
const path = require('path')
const { spawn } = require('child_process')


const PKG_DIR = 'pkg'
const TPLS_DIR = 'build_web_tpls'
const TPL_FILES = ['index.js', 'index.d.ts']


const buildWasm = () => new Promise((resolve, reject) => {
  const cmd = spawn('wasm-pack', ['build']);

  cmd.stdout.on('data', (data) => {
    console.log(data.toString())
  })

  cmd.stderr.on('data', (data) => {
    console.warn(data.toString())
  })

  cmd.on('close', (code) => {
    if (code !== 0) {
      reject(`wasp-pack build process exited with code ${code}`)
    } else {
      resolve()
    }
  })
})


function copyTpls() {
  return Promise.all(TPL_FILES.map(fName => new Promise((resolve, reject) => {
    fs.copyFile(
      path.resolve(TPLS_DIR, fName),
      path.resolve(PKG_DIR, fName),
      err => {
        if (err) {
          reject(err)
          return
        }
        resolve()
      }
    )
  })))
}


async function editPackageJson() {
  const pkgJsonPath = path.resolve('.', PKG_DIR, 'package.json')
  const pkg = require(pkgJsonPath)
  TPL_FILES.forEach(fName => !pkg.files.includes(fName) && pkg.files.push(fName))
  pkg.module = 'index.js'
  pkg.types = 'index.d.ts'

  fs.writeFileSync(pkgJsonPath, JSON.stringify(pkg, null, '  '))
}



async function start() {
  await buildWasm()
  await copyTpls()
  await editPackageJson()
}

start().catch(err => console.log(err))




