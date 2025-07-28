# SEO Implementation Documentation

## Overview
This document provides complete SEO implementations with inline code and normalized data structures that can be used across different projects and frameworks.

## Core SEO Data

### Personal/Brand Information
```json
{
  "username": "hmziqrs",
  "name": "hmziqrs",
  "title": "Senior Software Engineer",
  "yearsOfExperience": 9,
  "email": "hmziqrs@gmail.com",
  "skills": [
    "Flutter", "Dioxus", "React", "React Native",
    "Next.JS", "HonoJS", "AdonisJS", "Axum",
    "Docker", "CI/CD", "Architecture", "Animations"
  ],
  "websites": {
    "portfolio": "https://hmziq.rs",
    "cv": "https://cv.hmziq.rs"
  },
  "social": {
    "github": "hmziqrs",
    "linkedin": "hmziqrs",
    "twitter": "hmziqrs",
    "instagram": "hmziqrs",
    "youtube": "hmziqrs",
    "medium": "hmziqrs",
    "dev": "hmziqrs",
    "stackoverflow": null
  }
}
```

### SEO Configuration
```json
{
  "theme": {
    "themeColor": "#000000",
    "backgroundColor": "#000000",
    "textColor": "#ffffff"
  },
  "viewport": {
    "width": "device-width",
    "initialScale": 1,
    "maximumScale": 5,
    "userScalable": true
  },
  "seo": {
    "additionalKeywords": [
      "Software Engineer", "Frontend Developer",
      "Backend Developer", "Full-stack Developer",
      "Web Developer", "JavaScript Developer",
      "TypeScript Developer", "React Developer",
      "Node.js Developer", "Modern Web Development",
      "Software Architecture", "Web Performance",
      "User Experience", "Technical Leadership"
    ]
  },
  "robots": {
    "index": true,
    "follow": true,
    "googleBot": {
      "index": true,
      "follow": true,
      "maxVideoPreview": -1,
      "maxImagePreview": "large",
      "maxSnippet": -1
    }
  },
  "openGraph": {
    "type": "website",
    "locale": "en_US",
    "imageWidth": 1200,
    "imageHeight": 630,
    "imageType": "image/png"
  },
  "twitter": {
    "card": "summary_large_image"
  }
}
```

## Complete Metadata Implementation

### HTML Meta Tags
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <!-- Basic Meta Tags -->
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes">
  <title>hmziqrs - Senior Software Engineer</title>
  <meta name="description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development, TypeScript, React, and modern web technologies.">
  <meta name="keywords" content="hmziqrs, Senior Software Engineer, Flutter, Dioxus, React, React Native, Next.JS, HonoJS, AdonisJS, Axum, Docker, CI/CD, Architecture, Animations, Software Engineer, Frontend Developer, Backend Developer, Full-stack Developer, Web Developer, JavaScript Developer, TypeScript Developer, React Developer, Node.js Developer, Modern Web Development, Software Architecture, Web Performance, User Experience, Technical Leadership">
  <meta name="author" content="hmziqrs">
  <meta name="creator" content="hmziqrs">
  <meta name="publisher" content="hmziqrs">

  <!-- Theme Color -->
  <meta name="theme-color" content="#000000">

  <!-- Robots -->
  <meta name="robots" content="index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1">
  <meta name="googlebot" content="index, follow, max-video-preview:-1, max-image-preview:large, max-snippet:-1">

  <!-- Open Graph / Facebook -->
  <meta property="og:type" content="website">
  <meta property="og:locale" content="en_US">
  <meta property="og:url" content="https://hmziq.rs">
  <meta property="og:title" content="hmziqrs - Senior Software Engineer">
  <meta property="og:description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development.">
  <meta property="og:site_name" content="hmziq.rs">

  <!-- Twitter -->
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:url" content="https://hmziq.rs">
  <meta name="twitter:title" content="hmziqrs - Senior Software Engineer">
  <meta name="twitter:description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development.">
  <meta name="twitter:creator" content="@hmziqrs">

  <!-- Favicon -->
  <link rel="icon" type="image/x-icon" href="/favicon.ico">
  <link rel="icon" type="image/png" sizes="16x16" href="/fav/favicon-16x16.png">
  <link rel="icon" type="image/png" sizes="32x32" href="/fav/favicon-32x32.png">
  <link rel="apple-touch-icon" sizes="180x180" href="/fav/apple-touch-icon.png">

  <!-- Web Manifest -->
  <link rel="manifest" href="/fav/site.webmanifest">
</head>
</html>
```

### Structured Data (JSON-LD)
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Person",
  "name": "hmziqrs",
  "jobTitle": "Senior Software Engineer",
  "description": "Senior Software Engineer with 9 years of experience in full-stack development, specializing in modern web technologies.",
  "url": "https://hmziq.rs",
  "sameAs": [
    "https://github.com/hmziqrs",
    "https://linkedin.com/in/hmziqrs",
    "https://twitter.com/hmziqrs"
  ],
  "email": "hmziqrs@gmail.com",
  "knowsAbout": [
    "Flutter", "Dioxus", "React", "React Native",
    "Next.JS", "HonoJS", "AdonisJS", "Axum",
    "Docker", "CI/CD", "Architecture", "Animations"
  ]
}
</script>
```

## Technical SEO Files

### robots.txt
```txt
User-agent: *
Allow: /

Sitemap: https://hmziq.rs/sitemap.xml
```

### sitemap.xml
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://hmziq.rs</loc>
    <lastmod>2025-07-28T00:00:00.000Z</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>
```

### Web Manifest (site.webmanifest)
```json
{
  "name": "hmziqrs",
  "short_name": "hmziqrs",
  "icons": [
    {
      "src": "/fav/android-chrome-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/fav/android-chrome-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ],
  "theme_color": "#000000",
  "background_color": "#000000",
  "display": "standalone",
  "start_url": "/",
  "orientation": "portrait"
}
```

## Framework-Specific Implementations

### Next.js 13+ App Router
```typescript
import type { Metadata, Viewport } from 'next'

export const metadata: Metadata = {
  title: 'hmziqrs - Senior Software Engineer',
  description: 'Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development, TypeScript, React, and modern web technologies.',
  keywords: [
    'hmziqrs',
    'Senior Software Engineer',
    'Flutter', 'Dioxus', 'React', 'React Native',
    'Next.JS', 'HonoJS', 'AdonisJS', 'Axum',
    'Docker', 'CI/CD', 'Architecture', 'Animations',
    'Software Engineer', 'Frontend Developer',
    'Backend Developer', 'Full-stack Developer',
    'Web Developer', 'JavaScript Developer',
    'TypeScript Developer', 'React Developer',
    'Node.js Developer', 'Modern Web Development',
    'Software Architecture', 'Web Performance',
    'User Experience', 'Technical Leadership'
  ],
  authors: [{ name: 'hmziqrs', url: 'https://hmziq.rs' }],
  creator: 'hmziqrs',
  publisher: 'hmziqrs',
  metadataBase: new URL('https://hmziq.rs'),
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: 'https://hmziq.rs',
    title: 'hmziqrs - Senior Software Engineer',
    description: 'Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development.',
    siteName: 'hmziq.rs',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'hmziqrs - Senior Software Engineer',
    description: 'Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development.',
    creator: '@hmziqrs',
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
}

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 5,
  themeColor: '#000000',
}

// Sitemap generation
export async function sitemap(): Promise<MetadataRoute.Sitemap> {
  return [
    {
      url: 'https://hmziq.rs',
      lastModified: new Date(),
      changeFrequency: 'daily',
      priority: 1,
    },
  ]
}

// Robots.txt generation
export function robots(): MetadataRoute.Robots {
  return {
    rules: [
      {
        userAgent: '*',
        allow: ['/'],
      },
    ],
    sitemap: 'https://hmziq.rs/sitemap.xml',
  }
}
```

### React Helmet Implementation
```jsx
import { Helmet } from 'react-helmet'

function SEO() {
  return (
    <Helmet>
      <title>hmziqrs - Senior Software Engineer</title>
      <meta name="description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development, TypeScript, React, and modern web technologies." />
      <meta name="keywords" content="hmziqrs, Senior Software Engineer, Flutter, Dioxus, React, React Native, Next.JS, HonoJS, AdonisJS, Axum, Docker, CI/CD, Architecture, Animations, Software Engineer, Frontend Developer, Backend Developer, Full-stack Developer, Web Developer, JavaScript Developer, TypeScript Developer, React Developer, Node.js Developer, Modern Web Development, Software Architecture, Web Performance, User Experience, Technical Leadership" />
      <meta name="author" content="hmziqrs" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes" />
      <meta name="theme-color" content="#000000" />

      {/* Open Graph */}
      <meta property="og:type" content="website" />
      <meta property="og:locale" content="en_US" />
      <meta property="og:url" content="https://hmziq.rs" />
      <meta property="og:title" content="hmziqrs - Senior Software Engineer" />
      <meta property="og:description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development." />
      <meta property="og:site_name" content="hmziq.rs" />

      {/* Twitter */}
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content="hmziqrs - Senior Software Engineer" />
      <meta name="twitter:description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development." />
      <meta name="twitter:creator" content="@hmziqrs" />

      {/* JSON-LD */}
      <script type="application/ld+json">
        {JSON.stringify({
          "@context": "https://schema.org",
          "@type": "Person",
          "name": "hmziqrs",
          "jobTitle": "Senior Software Engineer",
          "description": "Senior Software Engineer with 9 years of experience in full-stack development, specializing in modern web technologies.",
          "url": "https://hmziq.rs",
          "sameAs": [
            "https://github.com/hmziqrs",
            "https://linkedin.com/in/hmziqrs",
            "https://twitter.com/hmziqrs"
          ],
          "email": "hmziqrs@gmail.com"
        })}
      </script>
    </Helmet>
  )
}
```

## Analytics Implementation

### Google Analytics 4 (GA4)
```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-XXXXXXXXXX');
</script>
```

### Firebase Analytics Setup
```javascript
import { initializeApp } from 'firebase/app'
import { getAnalytics, isSupported, logEvent } from 'firebase/analytics'
import { getPerformance } from 'firebase/performance'

const firebaseConfig = {
  apiKey: "your-api-key",
  authDomain: "your-auth-domain",
  projectId: "your-project-id",
  storageBucket: "your-storage-bucket",
  messagingSenderId: "your-sender-id",
  appId: "your-app-id",
  measurementId: "your-measurement-id"
}

export const initFirebase = async () => {
  try {
    const app = initializeApp(firebaseConfig)
    const analyticsSupported = await isSupported()

    if (analyticsSupported) {
      const perf = getPerformance(app)
      const analytics = getAnalytics(app)

      // Track page views
      logEvent(analytics, 'page_view', {
        page_path: window.location.pathname,
        page_location: window.location.href,
      })

      return { app, analytics, perf }
    }

    return { app, analytics: null, perf: null }
  } catch (error) {
    console.error('Firebase initialization error:', error)
    return { app: null, analytics: null, perf: null }
  }
}
```

## Social Media URLs

### Complete Social Platform URLs
```javascript
const socialLinks = {
  github: "https://github.com/hmziqrs",
  linkedin: "https://linkedin.com/in/hmziqrs",
  twitter: "https://twitter.com/hmziqrs",
  instagram: "https://instagram.com/hmziqrs",
  youtube: "https://youtube.com/@hmziqrs",
  medium: "https://medium.com/@hmziqrs",
  dev: "https://dev.to/hmziqrs",
  email: "mailto:hmziqrs@gmail.com",
  portfolio: "https://hmziq.rs",
  cv: "https://cv.hmziq.rs"
}
```

## Generated Content Examples

### Dynamic Title Generation
```javascript
const generateTitle = (name, title) => `${name} - ${title}`
// Output: "hmziqrs - Senior Software Engineer"
```

### Dynamic Description Generation
```javascript
const generateDescription = (name, title, years) =>
  `Personal landing page of ${name} - ${title} with ${years} years of experience in full-stack development, TypeScript, React, and modern web technologies.`
// Output: "Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development, TypeScript, React, and modern web technologies."
```

### Keywords Array
```javascript
const keywords = [
  // Personal
  "hmziqrs",
  "Senior Software Engineer",

  // Skills
  "Flutter", "Dioxus", "React", "React Native",
  "Next.JS", "HonoJS", "AdonisJS", "Axum",
  "Docker", "CI/CD", "Architecture", "Animations",

  // Additional SEO Keywords
  "Software Engineer", "Frontend Developer",
  "Backend Developer", "Full-stack Developer",
  "Web Developer", "JavaScript Developer",
  "TypeScript Developer", "React Developer",
  "Node.js Developer", "Modern Web Development",
  "Software Architecture", "Web Performance",
  "User Experience", "Technical Leadership"
]
```

## Best Practices Summary

1. **Use semantic HTML5 tags** for better structure
2. **Include all essential meta tags** in the head section
3. **Implement structured data** with JSON-LD for rich snippets
4. **Create XML sitemap** for search engine crawling
5. **Configure robots.txt** for crawler instructions
6. **Add Open Graph tags** for social media sharing
7. **Include Twitter Card tags** for Twitter previews
8. **Implement analytics** for tracking and insights
9. **Use descriptive, keyword-rich content** naturally
10. **Ensure mobile responsiveness** with proper viewport settings

## Quick Copy Templates

### Minimal SEO Setup
```html
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>hmziqrs - Senior Software Engineer</title>
<meta name="description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development, TypeScript, React, and modern web technologies.">
<meta property="og:title" content="hmziqrs - Senior Software Engineer">
<meta property="og:description" content="Personal landing page of hmziqrs - Senior Software Engineer with 9 years of experience in full-stack development.">
<meta name="twitter:card" content="summary_large_image">
```
