const { DistributionAPI } = require('helios-core/common')

const ConfigManager = require('./configmanager')

// Primary remote URL for the distribution index.
// This is hosted on GitHub so it is always reachable.
exports.REMOTE_DISTRO_URL = 'https://raw.githubusercontent.com/azfarbod50-blip/TCC-Launcher/master/distribution.json'

// Fallback remote URL (the original upstream).
const FALLBACK_DISTRO_URL = 'https://helios-files.geekcorner.eu.org/distribution.json'

const isDev = require('./isdev')

const launcherDirectory = ConfigManager.getLauncherDirectory()

// In dev mode, we can also fall back to the distribution.json
// sitting in the project root so the launcher works without any
// network access at all.
const devModeLocalPath = isDev ? require('path').resolve(__dirname, '../../..', 'distribution.json') : null

const api = new DistributionAPI(
    launcherDirectory,
    null, // Injected forcefully by the preloader.
    null, // Injected forcefully by the preloader.
    exports.REMOTE_DISTRO_URL,
    false
)

// Extend the API with a fallback URL list so that if the primary
// remote is unreachable we try the fallback before giving up.
if (typeof api._loadDistributionNullable === 'function') {
    const originalLoad = api._loadDistributionNullable.bind(api)
    let triedFallback = false

    api._loadDistributionNullable = async function() {
        const result = await originalLoad()
        if (result != null) {
            triedFallback = false
            return result
        }
        // Primary remote failed, try the fallback URL.
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
                // Fallback also failed, continue to local.
            }
        }
        // If both remotes failed, try the dev-mode local file.
        if (devModeLocalPath) {
            try {
                const fs = require('fs-extra')
                if (await fs.pathExists(devModeLocalPath)) {
                    const raw = await fs.readFile(devModeLocalPath, 'utf-8')
                    const parsed = JSON.parse(raw)
                    return parsed
                }
            } catch (e) {
                DistributionAPI.log.error('Dev-mode local distribution.json failed.', e)
            }
        }
        return null
    }
}

exports.DistroAPI = api