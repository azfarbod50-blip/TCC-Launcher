/**
 * mojangManifest.js
 */

const https = require('https')
const fs = require('fs')
const path = require('path')

const MANIFEST_URL = 'https://launchermeta.mojang.com/mc/game/version_manifest_v2.json'
const CACHE_FILE = path.join(__dirname, 'cache', 'version_manifest.json')
const CACHE_TTL_MS = 1000 * 60 * 60 * 6 // 6 ساعت

function getJSON (url) {
  return new Promise((resolve, reject) => {
    const req = https.get(url, { headers: { 'User-Agent': 'helios-fork-launcher' } }, res => {
      if (res.statusCode !== 200) {
        reject(new Error(`Request to ${url} failed with status ${res.statusCode}`))
        res.resume()
        return
      }
      let data = ''
      res.on('data', chunk => { data += chunk })
      res.on('end', () => {
        try {
          resolve(JSON.parse(data))
        } catch (err) {
          reject(err)
        }
      })
    })
    req.setTimeout(15000, () => {
      req.destroy(new Error('Request timed out after 15s'))
    })
    req.on('error', reject)
  })
}

async function fetchManifest ({ forceRefresh = false } = {}) {
  try {
    if (!forceRefresh && fs.existsSync(CACHE_FILE)) {
      const stat = fs.statSync(CACHE_FILE)
      const age = Date.now() - stat.mtimeMs
      if (age < CACHE_TTL_MS) {
        return JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'))
      }
    }
  } catch (err) {
    console.warn('[mojangManifest] cache read failed, refetching:', err.message)
  }

  const manifest = await getJSON(MANIFEST_URL)

  try {
    fs.mkdirSync(path.dirname(CACHE_FILE), { recursive: true })
    fs.writeFileSync(CACHE_FILE, JSON.stringify(manifest))
  } catch (err) {
    console.warn('[mojangManifest] cache write failed:', err.message)
  }

  return manifest
}

function filterVersions (manifest, types = ['release']) {
  return manifest.versions.filter(v => types.includes(v.type))
}

// Used as a stable fallback icon for every Mojang-injected "server". Must
// NEVER be null/undefined — Helios's UI interpolates this directly into an
// <img src="..."> and other file-path lookups, and a null value gets
// stringified to the literal text "null", which some parts of the app then
// try to open as a real file path (crashes on Launch).
const DEFAULT_ICON_URL = 'https://launchercontent.mojang.com/v2/icons/minecraft.png'

/**
 * تبدیل یک ورودی نسخه‌ی مویانگ به یک "Server" مینیمال
 * که Helios بتونه بدون خطا پارسش کنه.
 */
function toDistributionServer (mojangVersion) {
  return {
    id: mojangVersion.id,
    name: `Minecraft ${mojangVersion.id}`,
    description: `Vanilla ${mojangVersion.type}`,
    icon: DEFAULT_ICON_URL,
    version: '1.0.0',
    address: '127.0.0.1:25565', // آدرس واقعی نداره، فقط placeholder
    minecraftVersion: mojangVersion.id,
    discord: null,
    mainServer: false,
    autoconnect: false,
    javaOptions: {},
    modules: [] // بدون مدلودر - فقط وانیلا
  }
}

async function getAsDistributionServers ({ types = ['release'], forceRefresh = false } = {}) {
  const manifest = await fetchManifest({ forceRefresh })
  return filterVersions(manifest, types).map(toDistributionServer)
}

module.exports = {
  fetchManifest,
  filterVersions,
  toDistributionServer,
  getAsDistributionServers
}