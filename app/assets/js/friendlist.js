/**
 * Friend List Manager — TCC Client
 * Manage friend list with launcher badges for TCC users.
 */

const path = require('path')
const fs = require('fs-extra')

const logger = require('helios-core').LoggerUtil.getLogger('FriendList')

/**
 * Get the friend list file path for a user.
 */
function getFriendListPath(launcherDir) {
    return path.join(launcherDir, 'friendList.json')
}

/**
 * Load the friend list from disk.
 */
function loadFriendList(launcherDir) {
    const filePath = getFriendListPath(launcherDir)
    try {
        if (fs.existsSync(filePath)) {
            return JSON.parse(fs.readFileSync(filePath, 'utf8'))
        }
    } catch (err) {
        logger.error('Failed to load friend list:', err.message)
    }
    return { friends: [], pendingRequests: [] }
}

/**
 * Save the friend list to disk.
 */
function saveFriendList(launcherDir, data) {
    const filePath = getFriendListPath(launcherDir)
    try {
        fs.ensureDirSync(path.dirname(filePath))
        fs.writeFileSync(filePath, JSON.stringify(data, null, 4), 'utf8')
    } catch (err) {
        logger.error('Failed to save friend list:', err.message)
    }
}

/**
 * Add a friend by username.
 */
function addFriend(launcherDir, username, displayName) {
    const data = loadFriendList(launcherDir)
    const existing = data.friends.find(f => f.username.toLowerCase() === username.toLowerCase())
    if (existing) {
        return false // Already a friend
    }
    data.friends.push({
        username,
        displayName: displayName || username,
        addedAt: Date.now(),
        online: false,
        tccUser: true // Default to TCC user badge
    })
    saveFriendList(launcherDir, data)
    return true
}

/**
 * Remove a friend by username.
 */
function removeFriend(launcherDir, username) {
    const data = loadFriendList(launcherDir)
    const idx = data.friends.findIndex(f => f.username.toLowerCase() === username.toLowerCase())
    if (idx === -1) return false
    data.friends.splice(idx, 1)
    saveFriendList(launcherDir, data)
    return true
}

/**
 * Check if a user is a friend.
 */
function isFriend(launcherDir, username) {
    const data = loadFriendList(launcherDir)
    return data.friends.some(f => f.username.toLowerCase() === username.toLowerCase())
}

/**
 * Get all friends.
 */
function getFriends(launcherDir) {
    return loadFriendList(launcherDir).friends || []
}

module.exports = {
    loadFriendList,
    saveFriendList,
    addFriend,
    removeFriend,
    isFriend,
    getFriends,
    getFriendListPath
}
