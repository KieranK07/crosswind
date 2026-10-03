import { isFlatpak } from '$lib/api';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { platform } from '@tauri-apps/plugin-os';

class UpdateState {
	next: Update | null = $state(null);
	isChecking = $state(false);

	refresh = async () => {
		// Upstream publishes no macOS builds, so there is nothing to update to.
		if (this.isChecking || platform() === 'macos') return;

		this.isChecking = true;
		this.next = await check();
		this.isChecking = false;
	};
}

const updates = new UpdateState();
export default updates;
