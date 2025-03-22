pub mod calculator;
pub mod auth;
pub mod manager;

use auth::AuthScreen;
use calculator::Calculator;
use manager::PasswordManager;

pub enum AppState {
    Calculator,
    Auth,
    Manager,
}

pub struct AppController {
    pub state: AppState,
    pub calculator: Calculator,
    pub auth_screen: AuthScreen,
    pub password_manager: PasswordManager,
}

impl Default for AppController {
    fn default() -> Self {
        Self {
            state: AppState::Calculator,
            calculator: Calculator::default(),
            auth_screen: AuthScreen::default(),
            password_manager: PasswordManager::default(),
        }
    }
}
