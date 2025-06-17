use actix_web::{get, HttpResponse, Result};
use tracing::info;

#[get("/")]
pub async fn homepage() -> Result<HttpResponse> {
    info!("Homepage requested");
    
    let html_content = generate_homepage_html();
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

/// Generates the complete HTML for the homepage
fn generate_homepage_html() -> String {
    let svg_logo = format!(
        r#"<svg width="64" height="32" viewBox="0 0 128 64" xmlns="http://www.w3.org/2000/svg">
            <!-- Black background box -->
            <rect x="2" y="2" width="124" height="60" rx="6" fill="black" />
            <!-- Orange outline -->
            <rect x="2" y="2" width="124" height="60" rx="6" fill="none" stroke="{}" stroke-width="4"/>
            <!-- Orange arrow -->
            <text x="66" y="42"
                  font-family="Courier New, monospace"
                  font-size="28"
                  fill="{}"
                  text-anchor="middle">
              &lt;--
            </text>
        </svg>"#,
        "#FF6B35", "#FF6B35"
    );
    
    format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Backspace AS</title>
    <link rel="icon" type="image/svg+xml" href="/static/favicon.svg">
    <link rel="icon" type="image/png" href="/static/favicon-32x32.png">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet">
    <link rel="stylesheet" type="text/css" href="/static/styles.css">
</head>
<body>
    <div class="container">
        <header class="header">
            <div class="logo-section">
                <div class="logo">
                    <div class="logo-svg">
                        {}
                    </div>
                    <h1>Backspace AS</h1>
                </div>
            </div>
        </header>
        
        <main class="main-content">
            <div class="welcome-section">
                <h2>Welcome to Backspace AS</h2>
                <p class="tagline">Solutions are developing as we speak</p>
            </div>
        </main>
        
        <footer class="footer">
            <p>&copy; 2025 Backspace AS. All rights reserved.</p>
            <p class="footer-tagline">Making technology work for you</p>
        </footer>
    </div>
</body>
</html>"#,
        svg_logo
    )
}