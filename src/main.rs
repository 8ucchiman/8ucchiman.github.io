/*
 * FileName:        main
 * Author:          8ucchiman
 * CreatedDate:     2025-08-30 19:04:06
 * LastModified:    2025-08-31 22:19:14
 * Reference:       8ucchiman.github.io
 * Description:     ---
 */


use std::{fs, path::Path};

#[derive(Clone)]
struct Tab<'a> {
    key: &'a str,
    label: &'a str,
    description: &'a str,
    gif_url: &'a str,
    video_url: &'a str,
}

#[derive(Clone)]
struct Project<'a> {
    title: &'a str,
    image_url: &'a str,
    github_url: &'a str,
    demo_url: &'a str,
    tags: &'a [&'a str],
}

fn main() -> std::io::Result<()> {
    // キャッシュバスター（本番は Actions から ASSET_VER=github.sha を渡す）
    let ver = std::env::var("ASSET_VER").unwrap_or_else(|_| "dev".to_string());

    // メディア選択用（video 優先 → gif）
    let tabs = vec![
        Tab { key: "robotics", label: "robotics", description: "Robotics demos, embedded systems, and real-time CV.", gif_url: "assets/mugen.gif",              video_url: "" },
        Tab { key: "3d",       label: "3d render", description: "Procedural scenes, Blender, OpenGL/GLFW, path tracing.",       gif_url: "assets/samurai_champloo.gif", video_url: "" },
        Tab { key: "game",     label: "game",      description: "Live rigs, DSP experiments, DAW workflows.",                    gif_url: "",                           video_url: "" },
        Tab { key: "music",    label: "music",     description: "Live rigs, DSP experiments, DAW workflows.",                    gif_url: "",                           video_url: "" },
        Tab { key: "bio",      label: "bio",       description: "Who are you, 8ucchiman?",                                       gif_url: "",                           video_url: "" },
        Tab { key: "others",   label: "others",    description: "WIP prototypes, notes, utilities, experiments.",                 gif_url: "",                           video_url: "" },
    ];

    // サンプル Projects（適宜書き換えてください）
    let projects = vec![
        Project { title: "Path Tracer", image_url: "https://picsum.photos/seed/pathtracer/800/600", github_url: "https://github.com/you/pathtracer", demo_url: "https://example.com/pathtracer", tags: &["rust","3d","rendering"] },
        Project { title: "ESP32-CAM", image_url: "https://picsum.photos/seed/esp32/800/600", github_url: "https://github.com/you/esp32-cam", demo_url: "", tags: &["embedded","cv"] },
        Project { title: "OpenGL Viewer", image_url: "https://picsum.photos/seed/opengl/800/600", github_url: "https://github.com/you/opengl-viewer", demo_url: "", tags: &["opengl","3d"] },
        Project { title: "DSP Toys", image_url: "https://picsum.photos/seed/dsp/800/600", github_url: "https://github.com/you/dsp-toys", demo_url: "", tags: &["audio","dsp","rust"] },
    ];

    let out = Path::new("dist");
    fs::create_dir_all(out)?;
    fs::create_dir_all(out.join("assets"))?;
    fs::create_dir_all(out.join("assets/fonts"))?;
    fs::write(out.join(".nojekyll"), b"")?;
    fs::write(out.join("assets/style.css"), STYLE_CSS)?;
    fs::write(out.join("assets/app.js"), APP_JS)?;

    // Contact URLs (edit these to your actual profiles)
    let contact_github   = "https://github.com/8ucchiman";
    let contact_linkedin = "https://www.linkedin.com/in/your-handle";
    let contact_youtube  = "https://www.youtube.com/@your-channel";
    let contact_blog     = "https://8ucchiman.github.io";

    fs::write(
        out.join("index.html"),
        index_page(&tabs, &projects, &ver, contact_github, contact_linkedin, contact_youtube, contact_blog),
    )?;

    println!("\nOK: generated ./dist\nPreview: python3 -m http.server -d dist 8000\n");
    Ok(())
}

fn index_page(
    tabs: &[Tab],
    projects: &[Project],
    ver: &str,
    gh: &str,
    li: &str,
    yt: &str,
    blog: &str,
) -> String {
    let (media_html, has_media) = pick_media_html(tabs);
    let projects_json = projects_to_json(projects);

    // r##" ... "## にして、HTML内の `"#` を安全に扱う
    format!(r##"<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>8ucchiman | Portfolio</title>
<link rel="preload" as="style" href="assets/style.css?v={v}">
<link rel="stylesheet" href="assets/style.css?v={v}">
<meta name="color-scheme" content="light dark">
</head>
<body>
<div class="bg-orbs" aria-hidden="true"></div>

<!-- フルスクリーン巨大プレビュー（ブラウザ全面） -->
<section class="preview" id="home" aria-label="home">
  <!-- 3行固定の見出し（常時表示） -->
  <h2 class="headline">
    <span>Where are you</span>
    <span>going next,</span>
    <span>8ucchiman?</span>
  </h2>

  <!-- 単一メディアのみ表示 -->
  <div class="media" id="media">
    {media}
  </div>

  <div class="fade"></div>
</section>

<!-- スクロール時に出現するタブ式ナビ -->
<nav class="sticky-tabs" id="stickyTabs" role="navigation" aria-label="section tabs">
  <button data-target="#home"     class="tablink" aria-label="Go to home">home</button>
  <button data-target="#about"    class="tablink" aria-label="Go to about">about</button>
  <button data-target="#projects" class="tablink" aria-label="Go to projects">projects</button>
  <button data-target="#contact"  class="tablink" aria-label="Go to contact">contact</button>
</nav>

<!-- コンテンツセクション -->
<section class="section" id="about" aria-label="about me">
  <div class="container">
    <div class="about-grid">
      <div class="about-media">
        <div class="about-frame">
          <img src="assets/me.jpg" alt="8ucchiman portrait">
        </div>
      </div>
      <div class="about-text">
        <p class="about-kicker">Rust / Robotics / CV</p>
        <h3 class="about-title">About me</h3>
        <div class="about-card">
          <h4 class="about-name">8ucchiman</h4>
          <p class="about-role">Robotics engineer / Rust developer</p>
          <p class="about-bio">I build small, reliable tools and enjoy procedural graphics, embedded systems, and computer vision. This site is generated by a zero‑dependency Rust builder.</p>
          <div class="about-actions">
            <a class="btn" href="https://github.com/8ucchiman" target="_blank" rel="noreferrer">GitHub</a>
            <a class="btn ghost" href="https://8ucchiman.github.io" target="_blank" rel="noreferrer">Blog</a>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>

<section class="section" id="projects" aria-label="projects">
  <div class="container">
    <h3>projects</h3>
    <div class="proj-controls" id="proj-controls" aria-label="project filters"></div>
    <div class="proj-grid" id="proj-grid" aria-live="polite"></div>
  </div>
</section>

<section class="section" id="contact" aria-label="contact">
  <div class="container">
    
    <div class="contact-column">
    <div class="contact-links">
      <a class="icon-link" href="{gh}" target="_blank" rel="noreferrer">
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M12 .5a12 12 0 0 0-3.79 23.39c.6.11.82-.26.82-.58v-2.14c-3.34.73-4.04-1.61-4.04-1.61-.55-1.39-1.35-1.76-1.35-1.76-1.1-.75.08-.74.08-.74 1.22.09 1.86 1.27 1.86 1.27 1.08 1.86 2.83 1.32 3.52 1.01.11-.78.42-1.32.76-1.62-2.66-.3-5.47-1.33-5.47-5.92 0-1.31.47-2.39 1.25-3.23-.13-.31-.54-1.56.12-3.25 0 0 1.01-.32 3.3 1.23a11.5 11.5 0 0 1 6 0c2.3-1.55 3.3-1.23 3.3-1.23.66 1.69.25 2.94.12 3.25.78.84 1.25 1.92 1.25 3.23 0 4.6-2.81 5.61-5.49 5.91.43.37.81 1.1.81 2.22v3.29c0 .32.21.7.82.58A12 12 0 0 0 12 .5z"/></svg>
      </a>
      <a class="icon-link" href="{li}" target="_blank" rel="noreferrer">
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.226.792 24 1.771 24h20.454C23.205 24 24 23.226 24 22.271V1.729C24 .774 23.205 0 22.225 0ZM6.75 20.452H3.92V9H6.75v11.452ZM5.337 7.433a2.062 2.062 0 1 1 0-4.124 2.062 2.062 0 0 1 0 4.124ZM20.447 20.452h-3.554V14.74c0-1.361-.027-3.112-1.897-3.112-1.898 0-2.189 1.48-2.189 3.007v5.817H9.254V9h3.414v1.561h.049c.476-.9 1.637-1.848 3.372-1.848 3.605 0 4.266 2.372 4.266 5.455v6.284Z"/></svg>
      </a>
      <a class="icon-link" href="{yt}" target="_blank" rel="noreferrer">
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M23.5 6.2a3.5 3.5 0 0 0-2.46-2.48C19.2 3.2 12 3.2 12 3.2s-7.2 0-9.04.52A3.5 3.5 0 0 0 .5 6.2 36.8 36.8 0 0 0 0 12c0 1.92.18 3.84.5 5.8a3.5 3.5 0 0 0 2.46 2.48C4.8 20.8 12 20.8 12 20.8s7.2 0 9.04-.52a3.5 3.5 0 0 0 2.46-2.48c.32-1.95.5-3.87.5-5.8 0-1.92-.18-3.84-.5-5.8ZM9.6 15.5V8.5L15.8 12l-6.2 3.5Z"/></svg>
      </a>
      <a class="icon-link" href="{blog}" target="_blank" rel="noreferrer">
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M5 3h10a4 4 0 0 1 4 4v12a2 2 0 0 1-2 2H7V5a2 2 0 0 1-2-2Zm0 2v14h12V7a2 2 0 0 0-2-2H5Zm3 4h6v2H8V9Zm0 4h8v2H8v-2Z"/></svg>
      </a>
    </div>
    <div class="contact-meta">
      <p class="contact-line"><a class="mail-link" href="mailto:8ucchiman@gmail.com">8ucchiman@gmail.com</a></p>
      <p class="contact-credit">YUKI IWABUCHI &copy; 2025</p>
    </div>
    </div>
  </div>
</section>

{fallback_note}
<script>window.__PROJECTS__ = {projects_json};</script>
<script src="assets/app.js?v={v}" defer></script>
</body>
</html>
"##,
        v = ver,
        media = media_html,
        fallback_note = if has_media { String::new() } else { r#"<p class="desc">No media found. Put a GIF/MP4 under assets/ and set its path in the code.</p>"#.to_string() },
        gh = html_attr(gh),
        li = html_attr(li),
        yt = html_attr(yt),
        blog = html_attr(blog),
        projects_json = projects_json
    )
}

fn pick_media_html(tabs: &[Tab]) -> (String, bool) {
    // 1) video_url 優先
    if let Some(t) = tabs.iter().find(|t| !t.video_url.trim().is_empty()) {
        let v = html_attr(t.video_url);
        return (format!(r#"<video playsinline muted loop autoplay preload="metadata" src="{v}"></video>"#), true);
    }
    // 2) なければ gif_url
    if let Some(t) = tabs.iter().find(|t| !t.gif_url.trim().is_empty()) {
        let g = html_attr(t.gif_url);
        return (format!(r#"<img loading="lazy" src="{g}" alt="preview gif">"#), true);
    }
    // 3) プレースホルダ
    (r#"<div class="placeholder"></div>"#.to_string(), false)
}

// ---- helpers ----
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
fn html_attr(s: &str) -> String { html_escape(s).replace('\"', "&quot;") }

fn projects_to_json(ps: &[Project]) -> String {
    let mut s = String::from("[");
    for (i, p) in ps.iter().enumerate() {
        if i > 0 { s.push(','); }
        let tags = {
            let mut t = String::from("[");
            for (j, tg) in p.tags.iter().enumerate() {
                if j > 0 { t.push(','); }
                t.push_str(&format!("\"{}\"", html_attr(tg)));
            }
            t.push(']');
            t
        };
        s.push_str(&format!(
            "{{\"title\":\"{}\",\"image_url\":\"{}\",\"github_url\":\"{}\",\"demo_url\":\"{}\",\"tags\":{}}}",
            html_attr(p.title), html_attr(p.image_url), html_attr(p.github_url), html_attr(p.demo_url), tags
        ));
    }
    s.push(']');
    s
}

// ----------------- Embedded assets -----------------
const STYLE_CSS: &str = r#"
/* === mononoki を全体に適用（woff2 → ttf フォールバック） === */
@font-face {
  font-family: 'mononoki';
  src: url('fonts/MononokiNerdFont-Regular.woff2') format('woff2'),
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: 'mononoki';
  src: url('fonts/MononokiNerdFont-Bold.woff2') format('woff2'),
  font-weight: 700;
  font-style: normal;
  font-display: swap;
}

:root{
  --bg1:#0b1220;--bg2:#0b1020;--fg:#e2e8f0;--muted:#94a3b8;--ring:rgba(255,255,255,.1);
  /* 追加：モダンなグラデのアクセント色 */
  --ac1:#60a5fa; /* sky-400 */
  --ac2:#a78bfa; /* violet-400 */
  --ac3:#34d399; /* emerald-400 */
}
@media (prefers-color-scheme: light){:root{--bg1:#f8fafc;--bg2:#eef2ff;--fg:#0f172a;--muted:#475569;--ring:rgba(0,0,0,.06)}}

*{box-sizing:border-box}
html,body{height:100%}
html{scroll-behavior:smooth}
body{
  margin:0; font-family:'mononoki', monospace; font-size:16px; line-height:1.6; color:var(--fg);
  background:linear-gradient(120deg,var(--bg1),var(--bg2)) fixed;
}
button,input,select,textarea{font:inherit}
h1,h2,h3,h4,h5,h6,code,pre,.headline,.tablink{font-family:'mononoki', monospace}

/* 背景のぼかしオーブ */
.bg-orbs::before,.bg-orbs::after{content:"";position:fixed;inset:auto;filter:blur(60px);z-index:-1;border-radius:9999px}
.bg-orbs::before{top:-60px;left:-40px;width:280px;height:280px;background:rgba(16,185,129,.18)}
.bg-orbs::after{bottom:-80px;right:-60px;width:320px;height:320px;background:rgba(99,102,241,.16)}

/* フルスクリーンの巨大プレビュー */
.preview{
  position:relative; width:100vw; height:100vh; overflow:hidden; background:#082b4b;
}
.media, .media img, .media video{
  position:absolute; inset:0; width:100%; height:100%; object-fit:cover;
}

/* 3行見出し（縦中央寄せ・縦引き伸ばし・行間確保） */
.headline{
  position:absolute; left:1vw; top:0; bottom:0; z-index:3; pointer-events:none;
  display:flex; flex-direction:column; justify-content:center;
  font-weight:900; letter-spacing:-.02em; color:#fff; text-shadow:0 2px 14px rgba(0,0,0,.55);
  margin:0; padding:0;
}
.headline span{
  display:block; text-align:left;
  font-size:clamp(28px, 12vh, 22vh);
  line-height:1;
  margin:4vh 0; padding:0;
  transform:scaleY(1.5); transform-origin:left center; /* 縦に引き伸ばす */
}

/* 下部グラデで可読性UP */
.fade{position:absolute;left:0;right:0;bottom:0;height:120px;background:linear-gradient(0deg,rgba(0,0,0,.45),transparent)}

/* セクション */
.section{
  min-height:100vh; display:flex; align-items:center;
  border-top:1px solid var(--ring);
  background:linear-gradient(180deg, transparent, rgba(0,0,0,.04));
}
.container{max-width:1100px; margin:0 auto; padding:6vh 20px;}
.section h3{margin:0 0 12px; font-size:clamp(24px, 5vw, 40px); font-weight:900;}
.section p{margin:0; color:var(--muted)}
.desc{max-width:1100px;margin:10px auto 32px;padding:0 20px;opacity:.8}

/* === Projects grid === */
.proj-controls{ display:flex; gap:8px; flex-wrap:wrap; margin:10px 0 16px; }
.chip{ appearance:none; border:1px solid var(--ring); background:transparent; color:inherit; padding:6px 10px; border-radius:999px; cursor:pointer; font-weight:700; }
.chip.active{ background:linear-gradient(135deg, var(--ac1), var(--ac2)); color:#fff; border-color:transparent; }
.proj-grid{ display:grid; gap:14px; grid-template-columns:repeat(auto-fill, minmax(240px, 1fr)); }
.proj-card{ position:relative; overflow:hidden; border-radius:16px; background:rgba(255,255,255,.03); border:1px solid var(--ring); box-shadow:0 10px 22px var(--ring); }
.proj-card img{ width:100%; height:180px; object-fit:cover; display:block; filter:saturate(1.02); transition: transform .35s ease; }
.proj-card:hover img{ transform: scale(1.05); }
.proj-info{ padding:10px 12px; display:flex; align-items:center; justify-content:space-between; gap:8px; }
.proj-title{ font-weight:900; letter-spacing:-.01em; }
.proj-tags{ display:flex; gap:6px; flex-wrap:wrap; opacity:.8; }
.tag{ font-size:12px; border:1px solid var(--ring); border-radius:999px; padding:2px 6px; }
.proj-actions{ display:flex; gap:8px; }
.btn.small{ padding:6px 10px; border-radius:10px; font-weight:800; }

/* === Contact links === */
.contact-links{ display:flex; gap:10px; flex-wrap:wrap; margin-top:10px; justify-content:center; }
.icon-link{ display:inline-flex; align-items:center; justify-content:center; gap:0; padding:10px; width:42px; height:42px; border-radius:12px; border:1px solid var(--ring); background:rgba(255,255,255,.03); color:inherit; text-decoration:none; box-shadow:0 10px 22px var(--ring); }
.icon-link:hover{ background:linear-gradient(135deg, var(--ac1), var(--ac2)); color:#fff; border-color:transparent; }
.icon{ width:20px; height:20px; display:block; }

/* Compact contact section */
#contact{ min-height:auto; }
#contact .container{ padding:22px 20px; }
#contact h3{ font-size:clamp(18px,2.8vw,24px); margin-bottom:8px; }
.contact-links{ gap:12px; }
.icon-link{ padding:10px; width:80px; height:80px; border-radius:12px; box-shadow:0 8px 18px var(--ring); }
.icon{ width:70px; height:77px; }
/* small text under icons */
.contact-column{ display:flex; flex-direction:column; align-items:center; gap:48px; margin-top:8px; }
.contact-meta{ margin-top:0; text-align:center; display:flex; flex-direction:column; align-items:center; gap:40px; }
.contact-line{ margin:0; font-size:.8rem; font-weight:400; opacity:.9; }
.contact-credit{ margin:0; font-size:.9rem; opacity:.75; }
.mail-link{ color:inherit; text-decoration:none; border-bottom:1px dotted currentColor; font-size:1rem; font-weight:600; }
.mail-link:hover{ text-decoration:underline; }

/* スクロール時に出現するタブ（A: スライドダウン＋フェード） */
.sticky-tabs{
  position:fixed; top:12px; left:50%; transform:translate(-50%, -12px);
  display:flex; gap:8px;
  background:rgba(15,23,42,.55);
  -webkit-backdrop-filter: blur(8px); backdrop-filter: blur(8px);
  border:1px solid var(--ring); padding:8px; border-radius:999px; z-index:10;

  opacity:0; pointer-events:none;
  transition:transform .35s cubic-bezier(.22,.61,.36,1), opacity .25s ease;
}
.sticky-tabs.visible{ transform:translate(-50%, 0); opacity:1; pointer-events:auto; }

.tablink{
  appearance:none; border:0; border-radius:999px; padding:8px 14px;
  color:#fff; background:transparent; font-weight:800; cursor:pointer;
}
.tablink:hover{background:rgba(255,255,255,.08)}
.tablink:focus{outline:2px solid rgba(255,255,255,.35); outline-offset:2px}

.tablink.active{
  background: linear-gradient(135deg, var(--ac1), var(--ac2), var(--ac3));
  color:#fff;
  box-shadow:
    0 6px 18px rgba(0,0,0,.28),
    0 0 0 1px rgba(255,255,255,.18) inset;
  transform: translateY(-1px);
}

/* 低モーション設定への配慮 */
@media (prefers-reduced-motion: reduce){
  .sticky-tabs, .sticky-tabs.visible, .tablink{ transition:none !important; transform:none !important; filter:none !important; }
}

/* プレースホルダ */
.placeholder{ width:100%; height:100%; background:linear-gradient(135deg, rgba(148,163,184,.25), rgba(226,232,240,.35)); }

/* === About me layout === */
.about-grid{ display:grid; grid-template-columns:1fr; gap:18px; align-items:center; }
@media(min-width:900px){ .about-grid{ grid-template-columns: 1.1fr 1.3fr; } }
.about-media{ width:100%; }
.about-frame{ position:relative; border-radius:20px; overflow:hidden; background:linear-gradient(135deg,#111827,#0b0f1a); box-shadow:0 16px 40px var(--ring); aspect-ratio:4/3; }
.about-frame img{ position:absolute; inset:0; width:100%; height:100%; object-fit:cover; filter:saturate(1.02); }
.about-text{ padding:8px 6px; }
.about-kicker{ margin:0 0 6px; font-size:12px; letter-spacing:.12em; text-transform:uppercase; opacity:.7; }
.about-title{ margin:.1rem 0 10px; font-size:clamp(24px,3vw,36px); font-weight:900; letter-spacing:-.02em; }
.about-card{ border:1px solid var(--ring); border-radius:14px; padding:14px 16px; background:rgba(255,255,255,.03); box-shadow:0 6px 16px var(--ring); }
@media (prefers-color-scheme: light){ .about-card{ background:rgba(15,23,42,.03); } }
.about-name{ margin:.2rem 0 .2rem; font-size:clamp(18px,2.2vw,24px); font-weight:800; }
.about-role{ margin:0 0 .6rem; opacity:.8; font-size:.95rem; }
.about-bio{ margin:.2rem 0 1rem; line-height:1.7; opacity:.95; }
"#;

const APP_JS: &str = r#"
// スクロール量に応じてタブ表示切替＋アクティブハイライト＋クリックでスムーススクロール
(function(){
  const tabs = document.getElementById('stickyTabs');
  const home = document.getElementById('home');
  if (!tabs || !home) return;

  // --- 1) タブの表示/非表示（ヒーロー領域から離れたら表示） ---
  if ('IntersectionObserver' in window) {
    const io = new IntersectionObserver(([entry])=>{
      const mostlyVisible = entry.intersectionRatio > 0.6;
      tabs.classList.toggle('visible', !mostlyVisible);
    }, { threshold: [0, 0.6, 1] });
    io.observe(home);
  } else {
    const onScrollShowHide = ()=>{
      const y = window.scrollY || document.documentElement.scrollTop;
      tabs.classList.toggle('visible', y > window.innerHeight * 0.4);
    };
    window.addEventListener('scroll', onScrollShowHide, { passive:true });
    onScrollShowHide();
  }

  // --- 2) セクションに応じてアクティブなタブをハイライト ---
  const links = Array.from(tabs.querySelectorAll('.tablink'));
  const sections = links
    .map(btn => document.querySelector(btn.getAttribute('data-target')))
    .filter(Boolean);

  function setActive(btn){
    links.forEach(b => {
      const active = b === btn;
      b.classList.toggle('active', active);
      if (active) b.setAttribute('aria-current','page');
      else b.removeAttribute('aria-current');
    });
  }

  // 画面中央に最も近いセクションのタブをアクティブにする
  let ticking = false;
  function updateActive(){
    if (sections.length === 0) return;
    const mid = window.innerHeight / 2;
    let bestIdx = 0;
    let bestDist = Infinity;
    for (let i=0; i<sections.length; i++){
      const r = sections[i].getBoundingClientRect();
      const center = r.top + r.height/2;
      const dist = Math.abs(center - mid);
      if (dist < bestDist){ bestDist = dist; bestIdx = i; }
    }
    setActive(links[bestIdx]);
  }

  window.addEventListener('scroll', ()=>{
    if (ticking) return;
    ticking = true;
    requestAnimationFrame(()=>{ updateActive(); ticking = false; });
  }, { passive:true });

  // --- 3) クリックで対象セクションへスクロール＆即ハイライト ---
  links.forEach(btn=>{
    btn.addEventListener('click', ()=>{
      const target = btn.getAttribute('data-target');
      const el = document.querySelector(target);
      if (!el) return;
      try { el.scrollIntoView({ behavior:'smooth', block:'start' }); }
      catch { location.hash = target; }
      setActive(btn);
      // スムーススクロール後の最終位置で再評価
      setTimeout(updateActive, 700);
    });
  });

  // 初期状態
  updateActive();
})();

// Projects: filter chips + grid render
(function(){
  const data = (window.__PROJECTS__ || []);
  const grid = document.getElementById('proj-grid');
  const controls = document.getElementById('proj-controls');
  if (!grid || !controls || !Array.isArray(data) || data.length === 0) return;

  const allTags = Array.from(new Set(data.flatMap(p => Array.isArray(p.tags) ? p.tags : []))).sort();
  let active = 'all';

  function renderChips(){
    const tags = ['all', ...allTags];
    controls.innerHTML = tags.map(t => `<button class="chip${t===active?' active':''}" data-tag="${t}">${t}</button>`).join('');
    controls.querySelectorAll('.chip').forEach(btn=>{
      btn.addEventListener('click', ()=>{ active = btn.getAttribute('data-tag'); render(); });
    });
  }

  function card(p){
    const tags = (p.tags||[]).map(t=>`<span class=\"tag\">${t}</span>`).join('');
    const gh = p.github_url ? `<a class=\"btn small\" href=\"${p.github_url}\" target=\"_blank\" rel=\"noreferrer\">GitHub</a>` : '';
    const dm = p.demo_url ? `<a class=\"btn small ghost\" href=\"${p.demo_url}\" target=\"_blank\" rel=\"noreferrer\">Demo</a>` : '';
    const img = p.image_url ? `<img loading=\"lazy\" src=\"${p.image_url}\" alt=\"${p.title}\">` : '<div style="height:180px"></div>';
    return `<div class=\"proj-card\">${img}<div class=\"proj-info\"><div><div class=\"proj-title\">${p.title||''}</div><div class=\"proj-tags\">${tags}</div></div><div class=\"proj-actions\">${gh}${dm}</div></div></div>`;
  }

  function render(){
    renderChips();
    const list = active==='all' ? data : data.filter(p => (p.tags||[]).includes(active));
    grid.innerHTML = list.map(card).join('');
  }

  render();
})();
"#;
