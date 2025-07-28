# SEO Implementation for hmziq.xyz

## Project Overview
HMZIQ LABS is an experimental portfolio showcasing innovative projects in Flutter, Rust, React, and modern web technologies. The site features a dark, minimalist aesthetic with glitch-inspired visual effects.

## Domain & Brand Information
- **Domain**: hmziq.xyz
- **Site Title**: HMZIQ LABS - Experiments
- **Tagline**: "Experiments"
- **Theme**: Dark mode, glitch aesthetic, experimental tech lab
- **Primary Colors**: Black (#000000) background, White (#ffffff) text

## Core SEO Data

### Website Information
```json
{
  "domain": "hmziq.xyz",
  "siteName": "HMZIQ LABS",
  "title": "HMZIQ LABS - Experiments | Senior Software Engineer Portfolio",
  "shortTitle": "HMZIQ LABS",
  "description": "Experimental tech lab showcasing innovative projects in Flutter, Rust, React, and modern web technologies. Explore cutting-edge UI/UX experiments and cross-platform development.",
  "tagline": "Experiments",
  "author": "hmziqrs",
  "yearEstablished": 2025,
  "type": "portfolio"
}
```

### Skills & Technologies (from projects)
```json
{
  "languages": ["Rust", "TypeScript", "JavaScript", "Dart", "Go"],
  "frameworks": [
    "Dioxus", "Flutter", "React", "React Native", 
    "Next.js", "SvelteKit", "Remix"
  ],
  "technologies": [
    "WebAssembly", "WebRTC", "WebGL", "Canvas API",
    "Docker", "CI/CD", "TailwindCSS", "Animations"
  ],
  "specializations": [
    "Cross-platform Development",
    "UI/UX Experimentation", 
    "Real-time Applications",
    "Performance Optimization",
    "Interactive Animations"
  ]
}
```

### SEO Keywords
```json
{
  "primary": [
    "HMZIQ LABS",
    "hmziqrs",
    "Experimental Portfolio",
    "Tech Lab",
    "Senior Software Engineer"
  ],
  "technical": [
    "Flutter Developer", "Rust Developer", "React Developer",
    "Dioxus Framework", "WebAssembly Development",
    "Cross-platform Apps", "Real-time Web Apps",
    "UI/UX Experiments", "Interactive Animations"
  ],
  "projects": [
    "Flutter Apps", "Rust WebAssembly", "React Components",
    "Video Conferencing", "Real-time Collaboration",
    "Canvas Animations", "WebRTC Applications"
  ]
}
```

## Complete HTML Meta Tags Implementation

Add the following to your `index.html` inside the `<head>` tag:

```html
<!-- Basic Meta Tags -->
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes">
<title>HMZIQ LABS - Experiments | Senior Software Engineer Portfolio</title>
<meta name="description" content="Experimental tech lab showcasing innovative projects in Flutter, Rust, React, and modern web technologies. Explore cutting-edge UI/UX experiments and cross-platform development.">
<meta name="keywords" content="HMZIQ LABS, hmziqrs, Experimental Portfolio, Tech Lab, Senior Software Engineer, Flutter Developer, Rust Developer, React Developer, Dioxus Framework, WebAssembly Development, Cross-platform Apps, Real-time Web Apps, UI/UX Experiments, Interactive Animations">
<meta name="author" content="hmziqrs">
<meta name="creator" content="hmziqrs">
<meta name="publisher" content="hmziqrs">
<meta name="generator" content="Dioxus">

<!-- Theme & Display -->
<meta name="theme-color" content="#000000">
<meta name="color-scheme" content="dark">
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-status-bar-style" content="black">

<!-- Canonical URL -->
<link rel="canonical" href="https://hmziq.xyz/">

<!-- Robots -->
<meta name="robots" content="index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1">
<meta name="googlebot" content="index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1">

<!-- Open Graph / Facebook -->
<meta property="og:type" content="website">
<meta property="og:locale" content="en_US">
<meta property="og:url" content="https://hmziq.xyz/">
<meta property="og:site_name" content="HMZIQ LABS">
<meta property="og:title" content="HMZIQ LABS - Experiments | Senior Software Engineer Portfolio">
<meta property="og:description" content="Experimental tech lab showcasing innovative projects in Flutter, Rust, React, and modern web technologies.">
<meta property="og:image" content="https://hmziq.xyz/og-image.png">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:image:type" content="image/png">
<meta property="og:image:alt" content="HMZIQ LABS - Experimental Tech Portfolio">

<!-- Twitter Card -->
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:site" content="@hmziqrs">
<meta name="twitter:creator" content="@hmziqrs">
<meta name="twitter:url" content="https://hmziq.xyz/">
<meta name="twitter:title" content="HMZIQ LABS - Experiments">
<meta name="twitter:description" content="Experimental tech lab showcasing innovative projects in Flutter, Rust, React, and modern web technologies.">
<meta name="twitter:image" content="https://hmziq.xyz/og-image.png">
<meta name="twitter:image:alt" content="HMZIQ LABS - Experimental Tech Portfolio">

<!-- Favicon -->
<link rel="icon" type="image/x-icon" href="/favicon.ico">
<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">

<!-- Web Manifest -->
<link rel="manifest" href="/manifest.json">

<!-- Preconnect to external domains -->
<link rel="preconnect" href="https://www.gstatic.com">
<link rel="preconnect" href="https://firebaseapp.com">
<link rel="dns-prefetch" href="https://www.googletagmanager.com">
```

## Structured Data (JSON-LD)

Add this script before the closing `</body>` tag in `index.html`:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "ProfilePage",
  "mainEntity": {
    "@type": "Person",
    "name": "hmziqrs",
    "alternateName": "HMZIQ",
    "description": "Senior Software Engineer specializing in experimental web technologies, cross-platform development, and innovative UI/UX",
    "url": "https://hmziq.xyz",
    "sameAs": [
      "https://github.com/hmziqrs",
      "https://twitter.com/hmziqrs"
    ],
    "jobTitle": "Senior Software Engineer",
    "knowsAbout": [
      "Flutter Development",
      "Rust Programming",
      "React Development",
      "WebAssembly",
      "Cross-platform Development",
      "UI/UX Design",
      "Real-time Applications",
      "Web Performance Optimization"
    ],
    "owns": {
      "@type": "WebSite",
      "name": "HMZIQ LABS",
      "url": "https://hmziq.xyz",
      "description": "Experimental tech lab and portfolio"
    }
  },
  "breadcrumb": {
    "@type": "BreadcrumbList",
    "itemListElement": [
      {
        "@type": "ListItem",
        "position": 1,
        "name": "Home",
        "item": "https://hmziq.xyz"
      },
      {
        "@type": "ListItem",
        "position": 2,
        "name": "Projects",
        "item": "https://hmziq.xyz/#projects"
      },
      {
        "@type": "ListItem",
        "position": 3,
        "name": "Contact",
        "item": "https://hmziq.xyz/#contact"
      }
    ]
  }
}
</script>
```

## Web Manifest (manifest.json)

Create this file in the root directory:

```json
{
  "name": "HMZIQ LABS - Experiments",
  "short_name": "HMZIQ LABS",
  "description": "Experimental tech lab showcasing innovative projects",
  "start_url": "/",
  "display": "standalone",
  "theme_color": "#000000",
  "background_color": "#000000",
  "orientation": "portrait-primary",
  "icons": [
    {
      "src": "/icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/icon-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

## robots.txt Content

```txt
User-agent: *
Allow: /

# Sitemap location
Sitemap: https://hmziq.xyz/sitemap.xml

# Crawl-delay for responsible crawling
Crawl-delay: 1

# Disallow access to WASM build artifacts
Disallow: /target/
Disallow: /*.wasm$
```

## sitemap.xml Content

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <url>
    <loc>https://hmziq.xyz/</loc>
    <lastmod>2025-07-28T00:00:00+00:00</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://hmziq.xyz/#projects</loc>
    <lastmod>2025-07-28T00:00:00+00:00</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://hmziq.xyz/#contact</loc>
    <lastmod>2025-07-28T00:00:00+00:00</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
  </url>
</urlset>
```

## Enhanced Firebase Analytics

Your existing Firebase Analytics can be enhanced with custom events:

```javascript
// Track project views
logEvent(analytics, 'view_project', {
  project_name: projectName,
  project_tech: projectTechnology,
  project_category: category
});

// Track contact link clicks
logEvent(analytics, 'contact_click', {
  contact_method: method, // 'github', 'twitter', 'email', 'website'
  page_section: 'contact'
});

// Track filter usage
logEvent(analytics, 'filter_projects', {
  filter_type: filterType, // 'all', 'flutter', 'rust', 'web'
  project_count: filteredCount
});
```

## Performance & SEO Best Practices

### 1. Image Optimization
- Create an `og-image.png` (1200x630px) for social sharing
- Use WebP format for any future images
- Implement lazy loading for project images

### 2. Dioxus-Specific Considerations
- Ensure proper hydration for SSR (when implemented)
- Use semantic HTML elements in components
- Implement proper ARIA labels for accessibility

### 3. Content Guidelines
- Keep descriptions under 160 characters
- Use action-oriented language
- Include relevant keywords naturally
- Update lastmod in sitemap when content changes

### 4. Monitoring
- Set up Google Search Console
- Monitor Core Web Vitals
- Track Firebase Performance metrics
- Regular lighthouse audits

## Social Media Preview Text

### LinkedIn
```
HMZIQ LABS - Where experimental meets exceptional. Explore my latest projects in Flutter, Rust, and React. Building the future of cross-platform development, one experiment at a time. 🚀
```

### Twitter/X
```
🧪 HMZIQ LABS is live! Check out my experimental tech portfolio featuring projects in:
• Flutter & Dart
• Rust & WASM
• React & TypeScript
• Real-time applications

Explore → hmziq.xyz
```

### GitHub Profile
```
Senior Software Engineer | Building experimental tech at HMZIQ LABS
🔬 Portfolio: hmziq.xyz
💻 Stack: Flutter, Rust, React, TypeScript
🚀 Focus: Cross-platform development & UI/UX innovation
```

## Implementation Checklist

- [ ] Update index.html with all meta tags
- [ ] Create og-image.png (1200x630px)
- [ ] Create favicon set (16x16, 32x32, 180x180)
- [ ] Create manifest.json
- [ ] Create robots.txt
- [ ] Create sitemap.xml
- [ ] Add structured data script
- [ ] Enhance Firebase Analytics events
- [ ] Submit sitemap to Google Search Console
- [ ] Test with Google's Rich Results Test
- [ ] Validate Open Graph tags with Facebook Debugger
- [ ] Test Twitter Cards with Twitter Card Validator