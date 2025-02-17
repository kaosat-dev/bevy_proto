use bevy::asset::{Asset, AssetPath, Handle, HandleId};
use bevy::prelude::{FromReflect, Reflect};

/// Replacement type for asset handles in a [`Schematic::Input`] generated by the
/// [derive macro].
///
/// [`Schematic::Input`]: crate::schematics::Schematic::Input
/// [derive macro]: bevy_proto_derive::Schematic
#[derive(Clone, Debug, Eq, PartialEq, Hash, Reflect, FromReflect)]
pub enum ProtoAsset {
    /// The path to an asset relative to the `assets` directory.
    AssetPath(String),
    /// An existing [`HandleId`].
    HandleId(HandleId),
}

impl ProtoAsset {
    /// Returns the [`AssetPath`] pointed to by this enum, if any.
    pub fn to_asset_path(&self) -> Option<AssetPath<'_>> {
        match self {
            ProtoAsset::AssetPath(path) => Some(AssetPath::from(path)),
            ProtoAsset::HandleId(_) => None,
        }
    }

    /// Returns a [`ProtoAsset::HandleId`] containing a default [`Handle`] for `T`.
    ///
    /// This can be used as `#[reflect(default = "ProtoAsset::default_handle_id::<T>")]`
    /// to denote a default value for a [`ProtoAsset`] field.
    pub fn default_handle_id<T: Asset>() -> Self {
        Self::HandleId(Handle::<T>::default().id())
    }
}

impl<T: Asset> From<Handle<T>> for ProtoAsset {
    fn from(value: Handle<T>) -> Self {
        Self::HandleId(value.id())
    }
}
