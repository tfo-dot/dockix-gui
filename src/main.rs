// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::{ModelRc, VecModel, Model};
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // --- MOCK DATA FOR PROJECTS ---
    let all_projects = vec![
        Project {
            name: "dockix".into(),
            provider: "GitHub".into(),
            reference: "main".into(),
            status: "full".into(),
        },
        Project {
            name: "slint-ui".into(),
            provider: "GitHub".into(),
            reference: "v1.2.0".into(),
            status: "outdated".into(),
        },
        Project {
            name: "backend-api".into(),
            provider: "GitLab".into(),
            reference: "feat/new-auth".into(),
            status: "in-progress".into(),
        },
        Project {
            name: "legacy-docs".into(),
            provider: "SSH".into(),
            reference: "7a2b3c4".into(),
            status: "error".into(),
        },
    ];

    let all_projects = Rc::new(all_projects);
    let projects_model = Rc::new(VecModel::from((*all_projects).clone()));
    ui.set_projects(ModelRc::from(projects_model.clone()));

    // --- MOCK DATA FOR ADMIN LOGS ---
    let logs_data = vec![
        LogEntry {
            user: "admin".into(),
            time: "2024-05-20 10:00".into(),
            action: "Added project dockix".into(),
        },
        LogEntry {
            user: "john_doe".into(),
            time: "2024-05-20 11:15".into(),
            action: "Changed role for user tfo".into(),
        },
        LogEntry {
            user: "system".into(),
            time: "2024-05-20 12:00".into(),
            action: "Sync failed for legacy-docs".into(),
        },
    ];
    let logs_model = Rc::new(VecModel::from(logs_data));
    ui.set_admin_logs(ModelRc::from(logs_model));

    // --- CALLBACKS ---
    
    let ui_handle = ui.as_weak();
    ui.on_change_view(move |view_name| {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_current_view(view_name);
        }
    });

    let ui_handle = ui.as_weak();
    let projects_model_clone = projects_model.clone();
    ui.on_refresh_projects(move || {
        if let Some(ui) = ui_handle.upgrade() {
            projects_model_clone.push(Project {
                name: format!("new-project-{}", projects_model_clone.row_count()).into(),
                provider: "Local".into(),
                reference: "master".into(),
                status: "full".into(),
            });
        }
    });

    let ui_handle = ui.as_weak();
    let all_projects_clone = all_projects.clone();
    ui.on_search_projects(move |query| {
        if let Some(ui) = ui_handle.upgrade() {
            let query = query.to_lowercase();
            let filtered: Vec<Project> = all_projects_clone.iter()
                .filter(|p| p.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
            
            ui.set_projects(ModelRc::from(Rc::new(VecModel::from(filtered))));
        }
    });

    // --- LOGIN LOGIC ---
    let ui_handle = ui.as_weak();
    ui.on_login(move |ip, user, pass| {
        if let Some(ui) = ui_handle.upgrade() {
            println!("Login attempt: IP={}, User={}, Pass={}", ip, user, pass);
            // Mock authentication
            ui.set_is_logged_in(true);
            ui.set_current_view("user-dashboard".into());
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_login_as_guest(move || {
        if let Some(ui) = ui_handle.upgrade() {
            println!("Guest access granted");
            ui.set_is_logged_in(true); // Treat guest as "logged in" for UI visibility
            ui.set_current_view("user-dashboard".into());
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_logout(move || {
        if let Some(ui) = ui_handle.upgrade() {
            println!("Logged out");
            ui.set_is_logged_in(false);
            ui.set_current_view("login".into());
        }
    });

    ui.on_open_docs(|name| {
        println!("Opening documentation for: {}", name);
    });

    ui.run()?;

    Ok(())
}
