use crate::{Asset, AssetId, AssetLoadError, AssetPath, UntypedAssetId};
use bevy_ecs::event::BufferedEvent;
use bevy_reflect::Reflect;
use core::fmt::Debug;

/// A [`BufferedEvent`] emitted when a specific [`Asset`] fails to load.
///
/// For an untyped equivalent, see [`UntypedAssetLoadFailedEvent`].
#[derive(BufferedEvent, Clone, Debug)]
pub struct AssetLoadFailedEvent<A: Asset> {
    /// The stable identifier of the asset that failed to load.
    pub id: AssetId<A>,
    /// The asset path that was attempted.
    pub path: AssetPath<'static>,
    /// Why the asset failed to load.
    pub error: AssetLoadError,
}

impl<A: Asset> AssetLoadFailedEvent<A> {
    /// Converts this to an "untyped" / "generic-less" asset error event that stores the type information.
    pub fn untyped(&self) -> UntypedAssetLoadFailedEvent {
        self.into()
    }
}

/// An untyped version of [`AssetLoadFailedEvent`].
#[derive(BufferedEvent, Clone, Debug)]
pub struct UntypedAssetLoadFailedEvent {
    /// The stable identifier of the asset that failed to load.
    pub id: UntypedAssetId,
    /// The asset path that was attempted.
    pub path: AssetPath<'static>,
    /// Why the asset failed to load.
    pub error: AssetLoadError,
}

impl<A: Asset> From<&AssetLoadFailedEvent<A>> for UntypedAssetLoadFailedEvent {
    fn from(value: &AssetLoadFailedEvent<A>) -> Self {
        UntypedAssetLoadFailedEvent {
            id: value.id.untyped(),
            path: value.path.clone(),
            error: value.error.clone(),
        }
    }
}

/// [`BufferedEvent`]s that occur for a specific loaded [`Asset`], such as "value changed" events and "dependency" events.
#[expect(missing_docs, reason = "Documenting the id fields is unhelpful.")]
#[derive(BufferedEvent, Reflect)]
pub enum AssetEvent<A: Asset> {
    /// Emitted whenever an [`Asset`] is added.
    Added { id: AssetId<A> },
    /// Emitted whenever an [`Asset`] value is modified.
    Modified { id: AssetId<A> },
    /// Emitted whenever an [`Asset`] is removed.
    Removed { id: AssetId<A> },
    /// Emitted when the last [`super::Handle::Strong`] of an [`Asset`] is dropped.
    Unused { id: AssetId<A> },
    /// Emitted whenever an [`Asset`] has been fully loaded (including its dependencies and all "recursive dependencies").
    LoadedWithDependencies { id: AssetId<A> },
}

impl<A: Asset> AssetEvent<A> {
    /// Returns `true` if this event is [`AssetEvent::LoadedWithDependencies`] and matches the given `id`.
    pub fn is_loaded_with_dependencies(&self, asset_id: impl Into<AssetId<A>>) -> bool {
        matches!(self, AssetEvent::LoadedWithDependencies { id } if *id == asset_id.into())
    }

    /// Returns `true` if this event is [`AssetEvent::Added`] and matches the given `id`.
    pub fn is_added(&self, asset_id: impl Into<AssetId<A>>) -> bool {
        matches!(self, AssetEvent::Added { id } if *id == asset_id.into())
    }

    /// Returns `true` if this event is [`AssetEvent::Modified`] and matches the given `id`.
    pub fn is_modified(&self, asset_id: impl Into<AssetId<A>>) -> bool {
        matches!(self, AssetEvent::Modified { id } if *id == asset_id.into())
    }

    /// Returns `true` if this event is [`AssetEvent::Removed`] and matches the given `id`.
    pub fn is_removed(&self, asset_id: impl Into<AssetId<A>>) -> bool {
        matches!(self, AssetEvent::Removed { id } if *id == asset_id.into())
    }

    /// Returns `true` if this event is [`AssetEvent::Unused`] and matches the given `id`.
    pub fn is_unused(&self, asset_id: impl Into<AssetId<A>>) -> bool {
        matches!(self, AssetEvent::Unused { id } if *id == asset_id.into())
    }
}

impl<A: Asset> Clone for AssetEvent<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: Asset> Copy for AssetEvent<A> {}

impl<A: Asset> Debug for AssetEvent<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Added { id } => f.debug_struct("Added").field("id", id).finish(),
            Self::Modified { id } => f.debug_struct("Modified").field("id", id).finish(),
            Self::Removed { id } => f.debug_struct("Removed").field("id", id).finish(),
            Self::Unused { id } => f.debug_struct("Unused").field("id", id).finish(),
            Self::LoadedWithDependencies { id } => f
                .debug_struct("LoadedWithDependencies")
                .field("id", id)
                .finish(),
        }
    }
}

impl<A: Asset> PartialEq for AssetEvent<A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Added { id: l_id }, Self::Added { id: r_id })
            | (Self::Modified { id: l_id }, Self::Modified { id: r_id })
            | (Self::Removed { id: l_id }, Self::Removed { id: r_id })
            | (Self::Unused { id: l_id }, Self::Unused { id: r_id })
            | (
                Self::LoadedWithDependencies { id: l_id },
                Self::LoadedWithDependencies { id: r_id },
            ) => l_id == r_id,
            _ => false,
        }
    }
}

impl<A: Asset> Eq for AssetEvent<A> {}
