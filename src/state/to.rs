
use crate::state::ActiveSyncState;
use crate::state::MoveActiveSyncState;

pub trait ToLockState {
	fn move_sync_state(self) -> MoveActiveSyncState<Self> where Self: Sized;
	fn sync_state(&self) -> ActiveSyncState;
}

