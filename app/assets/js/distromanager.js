const { DistributionAPI } = require('helios-core/common')

const ConfigManager = require('./configmanager')

// Primary remote URL for the distribution index.
exports.REMOTE_DISTRO_URL = 'https://raw.githubusercontent.com/azfarbod50-blip/TCC-Launcher/master/distribution.json'

// Fallback remote URL (the original upstream).
const FALLBACK_DISTRO_URL = 'https://helios-files.geekcorner.eu.org/distribution.json'

const isDev = require('./isdev')
const path = require('path')
const fs = require('fs-extra')

const launcherDirectory = ConfigManager.getLauncherDirectory()

// In dev mode, fall back to the distribution.json in the project root.
// This allows the launcher to work offline during development.
const devModeLocalPath = isDev
    ? path.resolve(__dirname, '..', '..', '..', 'distribution.json')
    : null

const api = new DistributionAPI(
    launcherDirectory,
    null, // Injected forcefully by the preloader.
    null, // Injected forcefully by the preloader.
    exports.REMOTE_DISTRO_URL,
    isDev // Use dev mode in dev so pullLocal() is tried directly.
)

// Extend the API with a multi-URL fallback chain so that if the
// primary remote is unreachable the launcher still works.
if (typeof api._loadDistributionNullable === 'function') {
    const originalLoad = api._loadDistributionNullable.bind(api)
    let triedFallback = false

    api._loadDistributionNullable = async function () {
        const result = await originalLoad()
        if (result != null) {
            triedFallback = false
            return result
        }

        // Primary remote + local failed. Try the fallback URL.
        if (!triedFallback) {
            triedFallback = true
            DistributionAPI.log.info('Primary distro URL failed, trying fallback...')
            try {
                const got = require('got')
                const res = await got.get(FALLBACK_DISTRO_URL, {
                    responseType: 'json',
                    timeout: { request: 15000 }
                })
                if (res && res.body) {
                    triedFallback = false
                    return res.body
                }
            } catch (e) {
                // Fallback also failed, continue to local file.
            }
        }

        // Last resort: try the dev-mode local file (project root).
        if (devModeLocalPath) {
            try {
                if (await fs.pathExists(devModeLocalPath)) {
                    const raw = await fs.readFile(devModeLocalPath, 'utf-8')
                    return JSON.parse(raw)
                }
            } catch (e) {
                DistributionAPI.log.error('Dev-mode local distribution.json failed.', e)
            }
        }

        return null
    }
}

exports.DistroAPI = api