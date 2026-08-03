/**
 * Script for welcome.ejs
 */

/* global LauncherImporter, ConfigManager */

document.getElementById('welcomeButton').addEventListener('click', async e => {
    loginOptionsCancelEnabled(false)
    loginOptionsViewOnLoginSuccess = VIEWS.landing
    loginOptionsViewOnLoginCancel = VIEWS.loginOptions

    // Check for other launchers on first launch
    try {
        const detected = await LauncherImporter.detectLaunchers()
        if (detected.length > 0) {
            // Show import dialog
            showImportDialog(detected)
            return
        }
    } catch (err) {
        console.warn('Launcher detection failed:', err)
    }

    switchView(VIEWS.welcome, VIEWS.loginOptions)
})

/**
 * Show a dialog asking the user if they want to import from other launchers.
 */
function showImportDialog(detected) {
    const overlay = document.getElementById('overlayContainer')
    const content = document.getElementById('overlayContent')
    const title = document.getElementById('overlayTitle')
    const desc = document.getElementById('overlayDesc')
    const actionBtn = document.getElementById('overlayAcknowledge')
    const dismissBtn = document.getElementById('overlayDismiss')

    const names = detected.map(d => d.name).join(', ')

    title.innerHTML = 'Import from Other Launchers'
    desc.innerHTML = `We detected the following launchers on your system:<br><strong>${names}</strong><br><br>Would you like to import mods, servers, and settings from them?`
    actionBtn.textContent = 'Import All'
    dismissBtn.textContent = 'Skip'
    dismissBtn.style.display = 'inline-block'

    actionBtn.onclick = async () => {
        toggleOverlay(false)
        let totalMods = 0
        for (const launcher of detected) {
            try {
                const result = await LauncherImporter.importFromLauncher(launcher)
                totalMods += result.mods || 0
            } catch (err) {
                console.warn('Import failed for', launcher.name, err)
            }
        }
        // Continue to login after import
        switchView(VIEWS.welcome, VIEWS.loginOptions)
    }

    dismissBtn.onclick = () => {
        toggleOverlay(false)
        switchView(VIEWS.welcome, VIEWS.loginOptions)
    }

    toggleOverlay(true)
}