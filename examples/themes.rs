//! Theme demonstration example
//!
//! This example shows how to use different themes in VibeUI.

use vibeui::{App, Component, Label, Button, Container, Result};
use vibeui::style::{Theme, ThemeManager, builtin};
use vibeui::components::LayoutType;

fn main() -> Result<()> {
    // Initialize the library
    vibeui::init()?;
    
    println!("Starting VibeUI Theme Demo...");
    println!("Press ESC or Ctrl+C to exit");
    
    // Create theme manager
    let mut theme_manager = ThemeManager::new();
    
    // Add built-in themes
    theme_manager.add_theme(builtin::dark());
    theme_manager.add_theme(builtin::light());
    theme_manager.add_theme(builtin::high_contrast());
    theme_manager.add_theme(builtin::retro());
    
    // Create the application
    let mut app = App::new()?;
    
    // Create main container
    let mut main_container = Container::with_id("main")
        .with_layout(LayoutType::Vertical)
        .with_border(theme_manager.get_style("border").unwrap_or(&vibeui::style::Style::default()).clone())
        .with_uniform_padding(2);
    
    // Add title with current theme
    let current_theme = theme_manager.current_theme_name().unwrap_or("unknown");
    main_container.add_child("title", Label::with_id("title", 
        format!("🎨 VibeUI Theme Demo - {}", current_theme))
        .with_color(theme_manager.get_palette().map(|p| p.primary).unwrap_or(vibeui::style::Color::Cyan))
        .with_bold(true));
    
    // Add theme showcase
    create_theme_showcase(&mut main_container, &theme_manager);
    
    // Add theme switcher buttons
    create_theme_switcher(&mut main_container, &mut theme_manager);
    
    // Add color palette display
    create_color_palette(&mut main_container, &theme_manager);
    
    // Add the main container to the app
    app.add_component("main", main_container);
    
    // Run the application
    println!("Running theme demo...");
    match app.run() {
        Ok(_) => println!("Theme demo exited successfully"),
        Err(e) => eprintln!("Theme demo error: {}", e),
    }
    
    Ok(())
}

fn create_theme_showcase(container: &mut Container, theme_manager: &ThemeManager) {
    // Create showcase container
    let mut showcase = Container::with_id("showcase")
        .with_layout(LayoutType::Vertical)
        .with_uniform_padding(1);
    
    // Add various styled labels
    showcase.add_child("header", Label::with_id("header", "Header Style")
        .with_color(theme_manager.get_style("header").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Blue))
        .with_bold(true));
    
    showcase.add_child("title_text", Label::with_id("title_text", "Title Text")
        .with_color(theme_manager.get_style("title").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White))
        .with_bold(true));
    
    showcase.add_child("subtitle_text", Label::with_id("subtitle_text", "Subtitle Text")
        .with_color(theme_manager.get_style("subtitle").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White))
        .with_italic(true));
    
    showcase.add_child("body_text", Label::with_id("body_text", "Body text - This is how regular text appears in the current theme.")
        .with_color(theme_manager.get_style("body").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White)));
    
    showcase.add_child("caption_text", Label::with_id("caption_text", "Caption text (dimmed)")
        .with_color(theme_manager.get_style("caption").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Gray)));
    
    // Add status labels
    let mut status_container = Container::with_id("status_container")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(1);
    
    status_container.add_child("success", Label::with_id("success", "✓ Success")
        .with_color(theme_manager.get_style("success").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Green)));
    
    status_container.add_child("warning", Label::with_id("warning", "⚠ Warning")
        .with_color(theme_manager.get_style("warning").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Yellow)));
    
    status_container.add_child("error", Label::with_id("error", "✗ Error")
        .with_color(theme_manager.get_style("error").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Red)));
    
    status_container.add_child("info", Label::with_id("info", "ℹ Info")
        .with_color(theme_manager.get_style("info").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Cyan)));
    
    showcase.add_child("status_container", status_container);
    
    container.add_child("showcase", showcase);
}

fn create_theme_switcher(container: &mut Container, theme_manager: &mut ThemeManager) {
    // Create theme switcher container
    let mut switcher = Container::with_id("switcher")
        .with_layout(LayoutType::Vertical)
        .with_uniform_padding(1);
    
    switcher.add_child("switcher_title", Label::with_id("switcher_title", "Theme Switcher:")
        .with_color(theme_manager.get_style("title").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White))
        .with_bold(true));
    
    // Create button row for theme switching
    let mut button_row = Container::with_id("button_row")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(1);
    
    // Create buttons for each theme
    let themes = vec!["dark", "light", "high_contrast", "retro"];
    for theme_name in themes {
        let button = Button::with_id(format!("btn_{}", theme_name), 
            format!("🎨 {}", theme_name.replace("_", " ").to_uppercase()))
            .on_click(move |_button| {
                println!("Switching to theme: {}", theme_name);
                // In a real implementation, this would switch the theme
            });
        
        button_row.add_child(format!("btn_{}", theme_name), button);
    }
    
    switcher.add_child("button_row", button_row);
    container.add_child("switcher", switcher);
}

fn create_color_palette(container: &mut Container, theme_manager: &ThemeManager) {
    // Create color palette container
    let mut palette_container = Container::with_id("palette")
        .with_layout(LayoutType::Vertical)
        .with_uniform_padding(1);
    
    palette_container.add_child("palette_title", Label::with_id("palette_title", "Color Palette:")
        .with_color(theme_manager.get_style("title").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White))
        .with_bold(true));
    
    if let Some(palette) = theme_manager.get_palette() {
        // Create color display rows
        let colors = vec![
            ("Primary", palette.primary),
            ("Secondary", palette.secondary),
            ("Accent", palette.accent),
            ("Background", palette.background),
            ("Surface", palette.surface),
            ("Text", palette.text),
            ("Error", palette.error),
            ("Warning", palette.warning),
            ("Success", palette.success),
            ("Info", palette.info),
        ];
        
        for (name, color) in colors {
            let mut color_row = Container::with_id(format!("color_row_{}", name.to_lowercase()))
                .with_layout(LayoutType::Horizontal)
                .with_uniform_padding(0);
            
            // Color name
            color_row.add_child(format!("color_name_{}", name.to_lowercase()), 
                Label::with_id(format!("color_name_{}", name.to_lowercase()), 
                    format!("{}:", name))
                .with_color(theme_manager.get_style("body").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::White)));
            
            // Color sample (represented as a filled block)
            color_row.add_child(format!("color_sample_{}", name.to_lowercase()), 
                Label::with_id(format!("color_sample_{}", name.to_lowercase()), 
                    "████████")
                .with_color(color));
            
            // Color hex value
            color_row.add_child(format!("color_hex_{}", name.to_lowercase()), 
                Label::with_id(format!("color_hex_{}", name.to_lowercase()), 
                    color.to_hex())
                .with_color(theme_manager.get_style("caption").map(|s| s.foreground).flatten().unwrap_or(vibeui::style::Color::Gray)));
            
            palette_container.add_child(format!("color_row_{}", name.to_lowercase()), color_row);
        }
    }
    
    container.add_child("palette", palette_container);
}

/// Custom theme example
pub fn create_custom_theme() -> Result<()> {
    vibeui::init()?;
    
    // Create a custom theme
    let mut custom_theme = Theme::with_description("ocean", "Ocean theme with blue and teal colors")
        .with_author("VibeUI")
        .with_version("1.0.0");
    
    // Customize the color palette
    custom_theme.palette.primary = vibeui::style::Color::Rgb(0, 119, 190);
    custom_theme.palette.secondary = vibeui::style::Color::Rgb(0, 150, 136);
    custom_theme.palette.accent = vibeui::style::Color::Rgb(0, 188, 212);
    custom_theme.palette.background = vibeui::style::Color::Rgb(13, 71, 161);
    custom_theme.palette.surface = vibeui::style::Color::Rgb(21, 101, 192);
    custom_theme.palette.text = vibeui::style::Color::White;
    
    // Generate default styles
    custom_theme.generate_default_styles();
    
    // Create theme manager and add custom theme
    let mut theme_manager = ThemeManager::new();
    theme_manager.add_theme(custom_theme);
    theme_manager.set_current_theme("ocean");
    
    // Create a simple app to showcase the custom theme
    let mut app = App::new()?;
    
    let mut container = Container::with_id("main")
        .with_layout(LayoutType::Vertical)
        .with_uniform_padding(2);
    
    container.add_child("title", Label::with_id("title", "🌊 Custom Ocean Theme")
        .with_color(theme_manager.get_palette().unwrap().primary)
        .with_bold(true));
    
    container.add_child("description", Label::with_id("description", 
        "This is a custom theme with ocean-inspired colors.")
        .with_color(theme_manager.get_palette().unwrap().text));
    
    app.add_component("main", container);
    
    println!("Running custom theme demo...");
    app.run()?;
    
    Ok(())
}