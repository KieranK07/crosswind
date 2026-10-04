import { isFlatpak } from '$lib/api';
import { type Update } from '@tauri-apps/plugin-updater';

class UpdateState {
	next: Update | null = $state(null);
	isChecking = $state(false);

	refresh = async () => {
		// This fork has no update feed: upstream's would replace it with Gale without macOS and ROUNDS support.
	};
}

const updates = new UpdateState();
export default updates;
