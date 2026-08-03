/**
 * ClientPack — TCC Client bundled mod management.
 * These mods are downloaded when the user selects a TCC Client server.
 */

const path = require('path')

const logger = require('helios-core').LoggerUtil.getLogger('ClientPack')

/**
 * Client mods bundled with TCC Client.
 * These are installed into the instance's mods/ directory.
 */
const CLIENT_MODS = {
    // Performance mods
    sodium: {
        id: 'net.caffeinemc:sodium:0.6.7+mc1.21.11',
        name: 'Sodium',
        category: 'performance',
        description: 'Modern rendering engine for Minecraft — dramatically improves FPS',
        artifact: {
            url: 'https://maven.caffeinemc.net/releases/net/caffeinemc/sodium/0.6.7%2Bmc1.21.11/sodium-0.6.7+mc1.21.11.jar',
            size: 0,
            MD5: ''
        }
    },
    lithium: {
        id: 'me.jellysquid.mods:lithium:0.15.0+mc1.21.11',
        name: 'Lithium',
        category: 'performance',
        description: 'General-purpose server and client optimization mod',
        artifact: {
            url: 'https://maven.caffeinated.cc/releases/me/jellysquid/mods/lithium/0.15.0%2Bmc1.21.11/lithium-0.15.0+mc1.21.11.jar',
            size: 0,
            MD5: ''
        }
    },
    phosphor: {
        id: 'me.jellysquid.mods:phosphor:0.8.1+mc1.21.11',
        name: 'Phosphor',
        category: 'performance',
        description: 'Faster chunk loading and lighting engine',
        artifact: {
            url: 'https://maven.fabricmc.net/me/jellysquid/mods/phosphor/0.8.1%2Bmc1.21.11/phosphor-0.8.1+mc1.21.11.jar',
            size: 0,
            MD5: ''
        }
    },
    ferritecore: {
        id: 'net.caffeinemc:ferrite-core:7.1.2',
        name: 'FerriteCore',
        category: 'performance',
        description: 'Reduces memory usage significantly',
        artifact: {
            url: 'https://maven.caffeinemc.net/releases/net/caffeinemc/ferrite-core/7.1.2/ferrite-core-7.1.2.jar',
            size: 0,
            MD5: ''
        }
    },
    entityculling: {
        id: 'com.github.tr7zw:entityculling:1.7.1',
        name: 'EntityCulling',
        category: 'performance',
        description: 'Skips rendering of entities hidden behind walls — huge FPS boost',
        artifact: {
            url: 'https://maven.fabricmc.net/com/github/tr7zw/entityculling/1.7.1/entityculling-1.7.1.jar',
            size: 0,
            MD5: ''
        }
    },
    // Utility mods
    minimap: {
        id: 'maven.xaero:xaeros-minimap:25.4.0',
        name: 'Xaero\'s Minimap',
        category: 'utility',
        description: 'A minimap mod that shows the terrain and points of interest',
        artifact: {
            url: 'https://maven.xaero96.net/maven/xaero/minimap/25.4.0/xaeros-minimap-25.4.0.jar',
            size: 0,
            MD5: ''
        }
    },
    worldmap: {
        id: 'maven.xaero:xaeros-world-map:1.38.8',
        name: 'Xaero\'s World Map',
        category: 'utility',
        description: 'A world map mod that renders the full game world',
        artifact: {
            url: 'https://maven.xaero96.net/maven/xaero/world-map/1.38.8/xaeros-world-map-1.38.8.jar',
            size: 0,
            MD5: ''
        }
    },
    keystrokes: {
        id: 'com.mineblock:keystrokes:1.0.4',
        name: 'Keystrokes',
        category: 'hud',
        description: 'Displays WASD keys and mouse buttons on screen',
        artifact: {
            url: 'https://maven.mineblock.dev/maven/mineblock/keystrokes/1.0.4/keystrokes-1.0.4.jar',
            size: 0,
            MD5: ''
        }
    },
    minihud: {
        id: 'maven.fabricmc:mini-hud:1.21.1',
        name: 'MiniHUD',
        category: 'hud',
        description: 'Lightweight HUD showing FPS, coordinates, and other game info',
        artifact: {
            url: 'https://maven.fabricmc.net/net/fabricmc/mini-hud/1.21.1/mini-hud-1.21.1.jar',
            size: 0,
            MD5: ''
        }
    }
}

/**
 * Get the client mods directory for a given server instance.
 * @param {string} instanceDir
 * @param {string} serverId
 * @returns {string}
 */
function getModsDir(instanceDir, serverId) {
    return path.join(instanceDir, serverId, 'mods')
}

/**
 * Check if a server is a TCC Client server.
 * @param {Object} server
 * @returns {boolean}
 */
function isClientServer(server) {
    return server != null && server.rawServer != null && server.rawServer.id != null && server.rawServer.id.startsWith('tcc-client-')
}

/**
 * Get the client mod configuration from the config.
 * @param {Object} configManager
 * @param {string} serverId
 * @returns {Object} { modId: boolean }
 */
function getClientModsConfig(configManager, serverId) {
    const cfg = configManager.getClientModsConfig(serverId)
    return cfg || {}
}

/**
 * Set a single client mod's enabled state.
 * @param {Object} configManager
 * @param {string} serverId
 * @param {string} modKey
 * @param {boolean} enabled
 */
function setClientModEnabled(configManager, serverId, modKey, enabled) {
    configManager.setClientModEnabled(serverId, modKey, enabled)
}

/**
 * Get the list of enabled client mod keys.
 * @param {Object} configManager
 * @param {string} serverId
 * @returns {string[]}
 */
function getEnabledModKeys(configManager, serverId) {
    const cfg = getClientModsConfig(configManager, serverId)
    return Object.keys(CLIENT_MODS).filter(key => cfg[key] !== false)
}

module.exports = {
    CLIENT_MODS,
    isClientServer,
    getClientModsConfig,
    setClientModEnabled,
    getEnabledModKeys,
    getModsDir
}
