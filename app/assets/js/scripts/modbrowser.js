/**
 * Mod Browser UI Script — Fabric only
 */

/* global toggleOverlay, ModrinthAPI, $ */

function toggleModBrowser(toggleState) {
    toggleOverlay(toggleState, false, 'modBrowserContent')
    if (toggleState) {
        populateVersionFilter()
    }
}

function showModBrowserLoading() {
    $('#modBrowserResults').hide()
    $('#modBrowserEmpty').hide()
    $('#modBrowserLoading').show()
}

function showModBrowserEmpty() {
    $('#modBrowserLoading').hide()
    $('#modBrowserResults').hide()
    $('#modBrowserEmpty').show()
}

function showModBrowserResults() {
    $('#modBrowserLoading').hide()
    $('#modBrowserEmpty').hide()
    $('#modBrowserResults').show()
}

function renderModResults(mods) {
    if (!mods || mods.length === 0) {
        showModBrowserEmpty()
        return
    }

    let html = ''
    for (const mod of mods) {
        const icon = mod.icon_url || ''
        const name = mod.title || mod.slug
        const author = mod.author || 'Unknown'
        const desc = mod.description || ''
        const downloads = mod.downloads || 0
        const categories = (mod.categories || []).join(', ')
        const versions = (mod.versions || []).slice(0, 5).join(', ')

        html += `<div class='modBrowserListing' data-project-id='${mod.project_id}'>`
        html += `<img class='modBrowserListingIcon' src='${icon}' alt='${name}' onerror='this.style.display="none"' />`
        html += `<div class='modBrowserListingDetails'>`
        html += `<span class='modBrowserListingName'>${escapeHtml(name)}</span>`
        html += `<span class='modBrowserListingAuthor'>by ${escapeHtml(author)}</span>`
        html += `<span class='modBrowserListingDescription'>${escapeHtml(desc)}</span>`
        html += `<div class='modBrowserListingInfo'>`
        html += `<span class='modBrowserListingDownloads'>↓ ${formatNumber(downloads)}</span>`
        if (categories) {
            html += `<span class='modBrowserListingTags'>${escapeHtml(categories)}</span>`
        }
        html += `</div>`
        html += `<div class='modBrowserListingVersions'>Versions: ${escapeHtml(versions)}</div>`
        html += `</div>`
        html += `<div class='modBrowserListingActions'>`
        html += `<button class='modBrowserDownloadBtn' data-project-id='${mod.project_id}'>Install</button>`
        html += `</div>`
        html += `</div>`
    }

    $('#modBrowserResults').html(html)
    showModBrowserResults()

    $('.modBrowserDownloadBtn').on('click', async function (e) {
        e.stopPropagation()
        const projectId = $(this).data('project-id')
        await handleModDownload(projectId)
    })
}

async function handleModDownload(projectId) {
    const gameVersion = $('#modBrowserVersionFilter').val() || ''

    showModBrowserLoading()

    try {
        const versions = await ModrinthAPI.getModVersions(projectId, gameVersion || undefined)

        if (!versions || versions.length === 0) {
            showModBrowserEmpty()
            $('#modBrowserResults').html('<div class=\'modBrowserNoVersions\'>No compatible versions found.</div>')
            showModBrowserResults()
            return
        }

        const latestVersion = versions[0]
        const modsDir = getModsDirectory()

        if (!modsDir) {
            showModBrowserEmpty()
            $('#modBrowserResults').html('<div class=\'modBrowserError\'>Could not determine mods directory.</div>')
            showModBrowserResults()
            return
        }

        const filePath = await ModrinthAPI.downloadMod(latestVersion, modsDir)
        const p = window.require ? window.require('path') : path
        const fileName = p.basename(filePath)

        showModBrowserEmpty()
        $('#modBrowserResults').html(
            `<div class='modBrowserSuccess'>` +
            `<span>✅ Installed: ${escapeHtml(fileName)}</span>` +
            `<span class='modBrowserSuccessDetail'>Version: ${escapeHtml(latestVersion.name || latestVersion.version_number)}</span>` +
            `</div>`
        )
        showModBrowserResults()

    } catch (err) {
        console.error('Mod download error:', err)
        showModBrowserEmpty()
        $('#modBrowserResults').html(`<div class='modBrowserError'>Failed to install: ${escapeHtml(err.message)}</div>`)
        showModBrowserResults()
    }
}

function getModsDirectory() {
    try {
        const commonDir = ConfigManager.getCommonDirectory()
        if (commonDir) {
            const p = window.require ? window.require('path') : path
            return p.join(commonDir, 'mods')
        }
    } catch (e) { /* ignore */ }

    try {
        const instanceDir = ConfigManager.getInstanceDirectory()
        const selectedServer = ConfigManager.getSelectedServer()
        if (instanceDir && selectedServer) {
            const p = window.require ? window.require('path') : path
            return p.join(instanceDir, selectedServer, 'mods')
        }
    } catch (e) { /* ignore */ }

    return null
}

// All stable Fabric-supported Minecraft versions (Aug 2026)
const FABRIC_VERSIONS = [
    '1.21.11', '1.21.10', '1.21.9', '1.21.8', '1.21.7', '1.21.6',
    '1.21.5', '1.21.4', '1.21.3', '1.21.2', '1.21.1', '1.21',
    '1.20.6', '1.20.5', '1.20.4', '1.20.3', '1.20.2', '1.20.1', '1.20',
    '1.19.4', '1.19.3', '1.19.2', '1.19.1', '1.19',
    '1.18.2', '1.18.1', '1.18',
    '1.17.1', '1.17',
    '1.16.5', '1.16.4', '1.16.3', '1.16.2', '1.16.1', '1.16',
    '1.15.2', '1.15.1', '1.15',
    '1.14.4', '1.14.3', '1.14.2', '1.14.1', '1.14'
]

function populateVersionFilter() {
    const select = document.getElementById('modBrowserVersionFilter')
    if (!select) return
    // Keep the "Any Version" option, clear the rest
    select.innerHTML = '<option value="">Any Version</option>'
    for (const v of FABRIC_VERSIONS) {
        const opt = document.createElement('option')
        opt.value = v
        opt.textContent = v
        select.appendChild(opt)
    }
}

async function performModSearch() {
    const query = $('#modBrowserSearchInput').val().trim()
    const gameVersion = $('#modBrowserVersionFilter').val() || ''

    if (!query && !gameVersion) {
        await loadPopularMods(gameVersion)
        return
    }

    showModBrowserLoading()

    try {
        const mods = await ModrinthAPI.searchMods(query, gameVersion || undefined)
        renderModResults(mods)
    } catch (err) {
        console.error('Mod search error:', err)
        showModBrowserEmpty()
        $('#modBrowserResults').html(`<div class='modBrowserError'>Search failed: ${escapeHtml(err.message)}</div>`)
        showModBrowserResults()
    }
}

async function loadPopularMods(gameVersion) {
    showModBrowserLoading()
    try {
        const mods = await ModrinthAPI.getPopularMods(gameVersion || undefined, 20)
        renderModResults(mods)
    } catch (err) {
        console.error('Failed to load popular mods:', err)
        showModBrowserEmpty()
        $('#modBrowserResults').html(`<div class='modBrowserError'>Failed to load: ${escapeHtml(err.message)}</div>`)
        showModBrowserResults()
    }
}

function escapeHtml(str) {
    const div = document.createElement('div')
    div.appendChild(document.createTextNode(str))
    return div.innerHTML
}

function formatNumber(num) {
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

// --- Search Suggestions (Autocorrect) ---

let suggestionTimeout = null
let currentSuggestions = []

/**
 * Fetch search suggestions from Modrinth API as the user types.
 * Shows a dropdown of matching mod names for quick selection.
 */
async function fetchSuggestions(query) {
    if (!query || query.length < 2) {
        hideSuggestions()
        return
    }

    try {
        const mods = await ModrinthAPI.searchMods(query, null, 5)
        currentSuggestions = mods
        renderSuggestions(mods)
    } catch (err) {
        console.error('Suggestion error:', err)
        hideSuggestions()
    }
}

/**
 * Render the suggestion dropdown.
 */
function renderSuggestions(mods) {
    const container = document.getElementById('modBrowserSuggestions')
    const list = document.getElementById('modBrowserSuggestionsList')

    if (!container || !list) return

    if (!mods || mods.length === 0) {
        hideSuggestions()
        return
    }

    list.innerHTML = ''
    for (const mod of mods) {
        const icon = mod.icon_url || 'assets/images/icons/sevenstar.png'
        const item = document.createElement('div')
        item.className = 'modBrowserSuggestionItem'
        item.innerHTML = `<img class='modBrowserSuggestionIcon' src='${icon}' onerror='this.style.display="none"' />` +
                         `<span>${escapeHtml(mod.title || mod.slug)}</span>`
        item.addEventListener('click', () => {
            $('#modBrowserSearchInput').val(mod.title || mod.slug)
            hideSuggestions()
            performModSearch()
        })
        list.appendChild(item)
    }

    container.style.display = 'block'
}

/**
 * Hide the suggestion dropdown.
 */
function hideSuggestions() {
    const container = document.getElementById('modBrowserSuggestions')
    if (container) {
        container.style.display = 'none'
    }
    currentSuggestions = []
}

// --- Event Bindings ---
document.addEventListener('DOMContentLoaded', () => {
    const searchInput = document.getElementById('modBrowserSearchInput')
    if (searchInput) {
        searchInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault()
                hideSuggestions()
                performModSearch()
            } else if (e.key === 'Escape') {
                hideSuggestions()
            }
        })

        searchInput.addEventListener('input', (e) => {
            clearTimeout(suggestionTimeout)
            suggestionTimeout = setTimeout(() => {
                const query = e.target.value.trim()
                if (query.length >= 2) {
                    fetchSuggestions(query)
                } else {
                    hideSuggestions()
                }
            }, 300)
        })

        // Hide suggestions when clicking outside
        searchInput.addEventListener('blur', () => {
            setTimeout(() => hideSuggestions(), 200)
        })
    }

    const closeBtn = document.getElementById('modBrowserClose')
    if (closeBtn) {
        closeBtn.addEventListener('click', () => {
            toggleModBrowser(false)
        })
    }

    populateVersionFilter()
})

window.toggleModBrowser = toggleModBrowser
