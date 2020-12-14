#!/usr/bin/env node

const path = require( 'path' )
const fs = require( 'fs' )

const PKG_PATH = path.resolve( process.env.WEB_DIST_DIR, 'package.json' )

const pkg = require( PKG_PATH )
pkg.name = 'cubic-spline-rs'
pkg.module = 'index.js'
pkg.version = '0.9.4'
pkg.types = 'index.d.ts'
pkg.files.push( 'index.js', 'index.d.ts' )
pkg.keywords = JSON.parse( process.env.CARGO_PKG_KEYWORDS )
pkg.author = {
  "name": "Max Zommer",
  "email": "emgyrz@gmail.com",
  "url": "https://github.com/emgyrz"
}

fs.writeFileSync( PKG_PATH, JSON.stringify( pkg, null, '  ' ) )
