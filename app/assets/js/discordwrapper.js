/**
 * Discord Rich Presence for TCC Launcher
 * Shows "Playing TCC Launcher — Fabric 1.21.1" etc. on Discord
 */
const RPC = require('discord-rpc-patch')
const ConfigManager = require('./configmanager')
const { LoggerUtil } = require('helios-core')

const logger = LoggerUtil.getLogger('DiscordRPC')

// TCC's Application ID — replace with your own from https://discord.com/developers
const CLIENT_ID = 'YOUR_DISCORD_APP_ID_HERE'

let rpc = null
let rpcReady = false

/**
 * Initialize Discord RPC connection.
 */
exports.init = function() {
    try {
        RPC.register(CLIENT_ID)
        rpc = new RPC.Client({ transport: 'ipc' })

        rpc.on('ready', () => {
            rpcReady = true
            logger.info('Discord RPC connected.')
        })

        rpc.on('error', (err) => {
            logger.warn('Discord RPC error:', err.message)
            rpcReady = false
        })

        rpc.login({ clientId: CLIENT_ID }).catch((err) => {
            logger.warn('Discord RPC login failed (Discord may not be running):', err.message)
        })
    } catch(err) {
        logger.warn('Failed to initialize Discord RPC:', err.message)
    }
}

/**
 * Update the rich presence with current activity.
 * 
 * @param {string} state - The state line (e.g. "Fabric 1.20.1")
 * @param {string} details - The details line (e.g. "Browsing Mods")
 * @param {string} largeImageKey - Large image key
 * @param {string} largeImageText - Tooltip for large image
 */
exports.setActivity = function(state, details, largeImageKey = 'tcc-logo', largeImageText = 'TCC Launcher') {
    if(!rpc || !rpcReady) return

    try {
        rpc.setActivity({
            details: details || 'TCC Launcher',
            state: state || 'Idle',
            largeImageKey: largeImageKey,
            largeImageText: largeImageText,
            smallImageKey: 'minecraft',
            smallImageText: 'Minecraft',
            instance: false
        })
    } catch(err) {
        logger.warn('Failed to set Discord activity:', err.message)
    }
}

/**
 * Set activity for game launch.
 */
exports.setGameActivity = function(loaderType, mcVersion) {
    exports.setActivity(
        `${loaderType} ${mcVersion}`,
        'Playing Minecraft',
        'tcc-logo',
        'TCC Launcher'
    )
}

/**
 * Clear the activity.
 */
exports.clearActivity = function() {
    if(!rpc || !rpcReady) return
    try {
        rpc.clearActivity()
    } catch(err) {
        logger.warn('Failed to clear Discord activity:', err.message)
    }
}

/**
 * Disconnect from Discord.
 */
exports.destroy = function() {
    if(rpc) {
        try {
            rpc.destroy()
        } catch(err) {
            // Ignore
        }
        rpc = null
        rpcReady = false
    }
}
