//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Dashboard
//! component and an Search component for fullstack apps to be used in our app.

mod dashboard;
pub use dashboard::Dashboard;

mod search;
pub use search::Search;
