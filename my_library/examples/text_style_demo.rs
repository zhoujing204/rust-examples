// examples/text_style_demo.rs

use my_library::text_style::{style, Color};

fn main() {
    println!("\n🎨 Terminal Text Styling Demo\n");

    // Color Showcase with Description
    println!("=== Color Palette ===");
    println!("{} - For errors and critical warnings",
        style("Red").color(Color::Red).build());
    println!("{} - For success and completed tasks",
        style("Green").color(Color::Green).build());
    println!("{} - For warnings and important notes",
        style("Yellow").color(Color::Yellow).build());
    println!("{} - For information and links",
        style("Blue").color(Color::Blue).build());
    println!("{} - For special highlights and debugging",
        style("Magenta").color(Color::Magenta).build());
    println!("{} - For system messages and prompts",
        style("Cyan").color(Color::Cyan).build());
    println!("{} - For normal emphasized text",
        style("White").color(Color::White).build());

    // Real-world usage examples
    println!("\n=== Real-world Examples ===");

    // Console output
    println!("\n📋 Console Output:");
    println!("{} Database connection established",
        style("[OK]").color(Color::Green).bold().build());
    println!("{} Cache needs cleanup",
        style("[WARN]").color(Color::Yellow).bold().build());
    println!("{} Authentication failed",
        style("[ERROR]").color(Color::Red).bold().build());
    println!("{} Starting backup process",
        style("[INFO]").color(Color::Blue).bold().build());

    // Code diff example
    println!("\n📝 Code Diff:");
    println!("{} - const MAX_RETRY = 3;",
        style("-").color(Color::Red).build());
    println!("{} + const MAX_RETRY = 5;",
        style("+").color(Color::Green).build());

    // System status
    println!("\n💻 System Status:");
    println!("CPU: {}",
        style("87%").color(Color::Red).bold().build());
    println!("Memory: {}",
        style("45%").color(Color::Yellow).bold().build());
    println!("Disk: {}",
        style("23%").color(Color::Green).bold().build());

    // Background color examples
    println!("\n🎯 Highlighted Messages:");
    println!("{}",
        style(" CRITICAL ")
            .color(Color::BgRed)
            .bold()
            .build());
    println!("{}",
        style(" SUCCESS ")
            .color(Color::BgGreen)
            .bold()
            .build());
    println!("{}",
        style(" WARNING ")
            .color(Color::BgYellow)
            .bold()
            .build());
    println!("{}",
        style(" INFO ")
            .color(Color::BgBlue)
            .bold()
            .build());

    // Combined styling example
    println!("\n🔧 Advanced Formatting:");
    let title = style("Configuration Settings")
        .bold()
        .underline()
        .color(Color::Cyan)
        .build();
    println!("{}", title);
    println!("└─ {} {}",
        style("Debug Mode:").italic().build(),
        style("Enabled").color(Color::Green).bold().build()
    );
    println!("└─ {} {}",
        style("Environment:").italic().build(),
        style("Production").color(Color::Red).bold().build()
    );

    // Interactive prompt example
    println!("\n💭 Interactive Prompt Example:");
    println!("{} Do you want to proceed? {}",
        style("?").color(Color::Blue).bold().build(),
        style("[y/N]").color(Color::Cyan).build()
    );

    // Error stack trace
    println!("\n❌ Error Stack Trace:");
    println!("{} in module 'auth'",
        style("RuntimeError").color(Color::Red).bold().build()
    );
    println!("  {} verify_token()",
        style("at").color(Color::Blue).build()
    );
    println!("  {} line 42",
        style("in").color(Color::Blue).build()
    );

    // Loading states
    println!("\n⏳ Loading States:");
    println!("{} Installing dependencies...",
        style("●").color(Color::Blue).blink().build()
    );
    println!("{} Download complete",
        style("✓").color(Color::Green).bold().build()
    );
    println!("{} Upload failed",
        style("✗").color(Color::Red).bold().build()
    );

    // Table-like output
    println!("\n📊 Resource Usage:");
    println!("┌──────────┬────────┬────────┐");
    println!("│ Service  │ Status │ Memory │");
    println!("├──────────┼────────┼────────┤");
    println!("│ Web      │ {} │  124MB │",
        style("Active").color(Color::Green).build());
    println!("│ Database │ {} │  512MB │",
        style("Active").color(Color::Green).build());
    println!("│ Cache    │ {} │   64MB │",
        style("Down  ").color(Color::Red).build());
    println!("└──────────┴────────┴────────┘");

    println!("\n✨ Demo Complete!\n");
}