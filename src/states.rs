use bevy::prelude::*;

/// The high-level state of the app
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Setup,
    InMenu,
    Loading(LoadSource),
    InGame,
}

/// An enum to indicate what type of loading we need to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadSource {
    /// Generate a new map from seed and user configuration.
    Generation,
    /// Load from local map data.
    FromLocal,
    /// Load from remote map data, used in online mode.
    FromOnline,
}

/// A computed state of any possible [AppState::Loading].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AppStateLoading;

impl ComputedStates for AppStateLoading {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::Loading(_) => Some(AppStateLoading),
            _ => None,
        }
    }
}

pub fn complete_setup(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InMenu);
}
