/* global window, path, LoggerUtil */
/**
 * Modrinth API Module — Fabric only
 * Uses fetch() (Electron renderer safe, no require('got') needed)
 */

const logger = LoggerUtil.getLogger('Modrinth')

const BASE_URL = 'https://api.modrinth.com/v2'
const USER_AGENT = 'HeliosLauncher/1.0.0 (contact@helioslauncher.com)'

/**
 * Make a GET request to the Modrinth API.
 */
async function apiGet(endpoint, params = {}) {
    const url = new URL(`${BASE_URL}${endpoint}`)
    for (const [k, v] of Object.entries(params)) {
        url.searchParams.set(k, v)
    }
    try {
        const resp = await fetch(url.toString(), {
            headers: { 'User-Agent': USER_AGENT }
        })
        if (!resp.ok) throw new Error(`HTTP ${resp.status}`)
        return await resp.json()
    } catch (err) {
        logger.error(`Modrinth API error for ${endpoint}:`, err.message)
        throw err
    }
}

/**
 * Search for Fabric mods on Modrinth.
 */
async function searchMods(query, gameVersion, limit = 20, index = 'relevance') {
    const facets = [['project_type:mod'], ['categories:fabric']]
    if (gameVersion) {
        facets.push([`versions:${gameVersion}`])
    }
    const data = await apiGet('/search', {
        query: query || '',
        facets: JSON.stringify(facets),
        limit: limit.toString(),
        index: index
    })
    return data.hits || []
}

/**
 * Get available versions for a specific mod (Fabric only).
 */
async function getModVersions(projectId, gameVersion) {
    const params = { loaders: '["fabric"]' }
    if (gameVersion) {
        params.game_versions = `["${gameVersion}"]`
    }
    return await apiGet(`/project/${projectId}/version`, params)
}

/**
 * Download a mod .jar file to the mods directory.
 */
async function downloadMod(version, modsDir) {
    if (!version || !version.files || version.files.length === 0) {
        throw new Error('No files available for this version')
    }
    await new Promise((resolve, reject) => {
        const fs = window.require ? window.require('fs-extra') : require('fs-extra')
        fs.ensureDir(modsDir).then(resolve).catch(reject)
    })

    const fs = window.require ? window.require('fs-extra') : require('fs-extra')
    const fileToDownload = version.files.find(f => f.primary) || version.files[0]
    const fileName = fileToDownload.filename
    const destPath = path.join(modsDir, fileName)

    logger.info(`Downloading ${fileName}...`)

    const resp = await fetch(fileToDownload.url, {
        headers: { 'User-Agent': USER_AGENT }
    })
    if (!resp.ok) throw new Error(`Download failed: HTTP ${resp.status}`)

    const buffer = Buffer.from(await resp.arrayBuffer())
    await fs.writeFile(destPath, buffer)
    logger.info(`Downloaded ${fileName} to ${destPath}`)
    return destPath
}

/**
 * Get popular/featured Fabric mods.
 */
async function getPopularMods(gameVersion, limit = 20) {
    return searchMods('', gameVersion, limit, 'downloads')
}

/**
 * Get project details by ID.
 */
async function getProject(projectId) {
    return await apiGet(`/project/${projectId}`)
}

window.ModrinthAPI = {
    searchMods,
    getModVersions,
    downloadMod,
    getPopularMods,
    getProject
}
