/**
 * Launcher Importer
 * Detects other Minecraft launchers and copies mods/servers/versions
 * READ-ONLY on source — never deletes from other launchers
 */

/* global path, fs, os, ConfigManager, LoggerUtil */

const loggerImporter = LoggerUtil.getLogger('Importer')

const LAUNCHERS = [
    {
        name: 'Minecraft Launcher (Official)',
        dataPath: () => path.join(os.homedir(), '.minecraft'),
        hasMods: false,
        hasServers: true,
        hasVersions: true,
        hasLogs: true
    },
    {
        name: 'CurseForge',
        dataPath: () => {
            const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local')
            const cfPath = path.join(base, 'CurseForge', 'Minecraft', 'Instances')
            if (fs.existsSync(cfPath)) return cfPath
            return null
        },
        hasMods: true,
        hasServers: false,
        hasVersions: false,
        hasLogs: false,
        isMultiInstance: true
    },
    {
        name: 'Modrinth App',
        dataPath: () => {
            const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local')
            const mrPath = path.join(base, 'ModrinthApp', 'profiles')
            if (fs.existsSync(mrPath)) return mrPath
            return null
        },
        hasMods: true,
        hasServers: false,
        hasVersions: false,
        hasLogs: false,
        isMultiInstance: true
    },
    {
        name: 'Prism Launcher',
        dataPath: () => {
            const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local')
            const prismPath = path.join(base, 'PrismLauncher', 'instances')
            if (fs.existsSync(prismPath)) return prismPath
            // Linux path fallback
            const linuxPath = path.join(os.homedir(), '.local', 'share', 'PrismLauncher', 'instances')
            if (fs.existsSync(linuxPath)) return linuxPath
            return null
        },
        hasMods: true,
        hasServers: false,
        hasVersions: false,
        hasLogs: false,
        isMultiInstance: true
    },
    {
        name: 'PolyMC',
        dataPath: () => {
            const base = process.env.LOCALAPPDATA || path.join(os.homedir(), 'AppData', 'Local')
            const pmcPath = path.join(base, 'PolyMC', 'instances')
            if (fs.existsSync(pmcPath)) return pmcPath
            return null
        },
        hasMods: true,
        hasServers: false,
        hasVersions: false,
        hasLogs: false,
        isMultiInstance: true
    },
    {
        name: 'HMCL',
        dataPath: () => {
            const hmclPath = path.join(os.homedir(), '.hmcl')
            if (fs.existsSync(hmclPath)) return hmclPath
            return null
        },
        hasMods: true,
        hasServers: true,
        hasVersions: true,
        hasLogs: true
    },
    {
        name: 'BakXL',
        dataPath: () => {
            const base = process.env.APPDATA || path.join(os.homedir(), 'AppData', 'Roaming')
            const bakxlPath = path.join(base, '.bakxl')
            if (fs.existsSync(bakxlPath)) return bakxlPath
            return null
        },
        hasMods: true,
        hasServers: false,
        hasVersions: false,
        hasLogs: false
    },
    {
        name: 'TLauncher',
        dataPath: () => {
            const tlPath = path.join(os.homedir(), '.tlauncher')
            if (fs.existsSync(tlPath)) return tlPath
            return null
        },
        hasMods: true,
        hasServers: true,
        hasVersions: true,
        hasLogs: true
    }
]

/**
 * Scan for installed launchers on this system.
 * @returns {Promise<Array>} Array of detected launcher objects
 */
async function detectLaunchers() {
    const detected = []
    for (const launcher of LAUNCHERS) {
        try {
            const dataPath = launcher.dataPath()
            if (dataPath && fs.existsSync(dataPath)) {
                detected.push({
                    name: launcher.name,
                    path: dataPath,
                    hasMods: launcher.hasMods,
                    hasServers: launcher.hasServers,
                    hasVersions: launcher.hasVersions,
                    hasLogs: launcher.hasLogs,
                    isMultiInstance: launcher.isMultiInstance || false
                })
                loggerImporter.info(`Detected: ${launcher.name} at ${dataPath}`)
            }
        } catch (e) {
            // Skip launcher on error
        }
    }
    return detected
}

/**
 * Copy a directory recursively (READ-ONLY on source).
 * @param {string} src - Source directory
 * @param {string} dest - Destination directory
 */
async function copyDir(src, dest) {
    if (!fs.existsSync(src)) return
    await fs.ensureDir(dest)

    const entries = await fs.readdir(src, { withFileTypes: true })
    for (const entry of entries) {
        const srcPath = path.join(src, entry.name)
        const destPath = path.join(dest, entry.name)

        if (entry.isDirectory()) {
            await copyDir(srcPath, destPath)
        } else {
            if (!fs.existsSync(destPath)) {
                await fs.copyFile(srcPath, destPath)
            }
        }
    }
}

/**
 * Import mods from a detected launcher.
 * @param {object} launcher - Detected launcher object
 * @param {string} instancePath - TCC instance directory
 * @returns {Promise<number>} Number of files copied
 */
async function importMods(launcher, instancePath) {
    const modsDir = path.join(instancePath, 'mods')
    await fs.ensureDir(modsDir)
    let count = 0

    const scanPaths = [
        path.join(launcher.path, 'mods'),
        path.join(launcher.path, 'mod')
    ]

    for (const srcMods of scanPaths) {
        if (!fs.existsSync(srcMods)) continue

        const entries = await fs.readdir(srcMods, { withFileTypes: true })
        for (const entry of entries) {
            if (entry.isFile() && entry.name.endsWith('.jar')) {
                const destPath = path.join(modsDir, entry.name)
                if (!fs.existsSync(destPath)) {
                    await fs.copyFile(path.join(srcMods, entry.name), destPath)
                    count++
                }
            }
        }
    }

    // Multi-instance launchers (CurseForge, Modrinth, Prism)
    if (launcher.isMultiInstance) {
        const instances = await fs.readdir(launcher.path, { withFileTypes: true })
        for (const inst of instances) {
            if (!inst.isDirectory()) continue
            const instMods = path.join(launcher.path, inst.name, 'mods')
            if (!fs.existsSync(instMods)) continue

            const entries = await fs.readdir(instMods, { withFileTypes: true })
            for (const entry of entries) {
                if (entry.isFile() && entry.name.endsWith('.jar')) {
                    const destPath = path.join(modsDir, entry.name)
                    if (!fs.existsSync(destPath)) {
                        await fs.copyFile(path.join(instMods, entry.name), destPath)
                        count++
                    }
                }
            }
        }
    }

    loggerImporter.info(`Imported ${count} mods from ${launcher.name}`)
    return count
}

/**
 * Import servers from a detected launcher.
 * @param {object} launcher - Detected launcher object
 * @param {string} instancePath - TCC instance directory
 * @returns {Promise<number>} Number of servers imported
 */
async function importServers(launcher, instancePath) {
    const serversDat = path.join(launcher.path, 'servers.dat')
    if (!fs.existsSync(serversDat)) return 0

    const destServers = path.join(instancePath, 'servers.dat')
    if (!fs.existsSync(destServers)) {
        await fs.copyFile(serversDat, destServers)
        loggerImporter.info(`Imported servers.dat from ${launcher.name}`)
        return 1
    }
    return 0
}

/**
 * Import versions from a detected launcher.
 * @param {object} launcher - Detected launcher object
 * @returns {Promise<number>} Number of versions imported
 */
async function importVersions(launcher) {
    const commonDir = ConfigManager.getCommonDirectory()
    const srcVersions = path.join(launcher.path, 'versions')
    if (!fs.existsSync(srcVersions)) return 0

    const destVersions = path.join(commonDir, 'versions')
    let count = 0

    const entries = await fs.readdir(srcVersions, { withFileTypes: true })
    for (const entry of entries) {
        if (entry.isDirectory()) {
            const srcVer = path.join(srcVersions, entry.name)
            const destVer = path.join(destVersions, entry.name)
            if (!fs.existsSync(destVer)) {
                await copyDir(srcVer, destVer)
                count++
            }
        }
    }

    loggerImporter.info(`Imported ${count} versions from ${launcher.name}`)
    return count
}

/**
 * Import everything from a detected launcher.
 * @param {object} launcher - Detected launcher object
 * @returns {Promise<object>} Import results
 */
async function importFromLauncher(launcher) {
    const results = { mods: 0, servers: 0, versions: 0 }

    try {
        const instanceDir = ConfigManager.getInstanceDirectory()
        const selectedServer = ConfigManager.getSelectedServer()
        const instancePath = path.join(instanceDir, selectedServer)

        if (launcher.hasMods) {
            results.mods = await importMods(launcher, instancePath)
        }
        if (launcher.hasServers) {
            results.servers = await importServers(launcher, instancePath)
        }
        if (launcher.hasVersions) {
            results.versions = await importVersions(launcher)
        }
    } catch (e) {
        loggerImporter.error(`Import failed for ${launcher.name}:`, e.message)
    }

    return results
}

window.LauncherImporter = {
    detectLaunchers,
    importFromLauncher
}
