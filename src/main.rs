/*
 * FileName:        main
 * Author:          8ucchiman
 * CreatedDate:     2025-08-30 19:04:06
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */



use std::{fs, path::Path};

#[derive(Clone)]
struct Tab<'a> {
    key: &'a str,         // 旧タブ由来。メディア選択にのみ使用
    label: &'a str,
    description: &'a str,
    gif_url: &'a str,     // .gif（任意）
    video_url: &'a str,   // .mp4/.webm（任意）
}

fn main() -> std::io::Result<()> {
    // メディア選択用（video優先→gif）
    let tabs = vec![
        Tab { key: "robotics", label: "robotics", description: "Robotics demos, embedded systems, and real-time CV.", gif_url: "assets/mugen.gif",              video_url: "" },
        Tab { key: "3d",       label: "3d render", description: "Procedural scenes, Blender, OpenGL/GLFW, path tracing.",       gif_url: "assets/samurai_champloo.gif", video_url: "" },
        Tab { key: "game",     label: "game",      description: "Live rigs, DSP experiments, DAW workflows.",                    gif_url: "",                           video_url: "" },
        Tab { key: "music",    label: "music",     description: "Live rigs, DSP experiments, DAW workflows.",                    gif_url: "",                           video_url: "" },
        Tab { key: "bio",      label: "bio",       description: "Who are you, 8ucchiman?",                                       gif_url: "",                           video_url: "" },
        Tab { key: "others",   label: "others",    description: "WIP prototypes, notes, utilities, experiments.",                 gif_url: "",                           video_url: "" },
    ];

    let out = Path::new("dist");
    fs::create_dir_all(out)?;
    fs::create_dir_all(out.join("assets"))?;
    fs::create_dir_all(out.join("assets/fonts"))?;
    fs::write(out.join(".nojekyll"), b"")?;
    fs::write(out.join("assets/style.css"), STYLE_CSS)?;
    fs::write(out.join("assets/app.js"), APP_JS)?;
    fs::write(out.join("index.html"), index_page(&tabs))?;

    println!("\nOK: generated ./dist\nPreview: python3 -m http.server -d dist 8000\n");
    Ok(())
}

fn index_page(tabs: &[Tab]) -> String {
    let (media_html, has_media) = pick_media_html(tabs);

    // NOTE: r##" ... "## にして、HTML内の `"#` に耐性を持たせる
    format!(r##"<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>8ucchiman | Portfolio</title>
<link rel="preload" as="style" href="assets/style.css">
<link rel="stylesheet" href="assets/style.css">
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
    <h3>about me</h3>
    <p>Short bio / skills / current focus. (Replace this with your actual content.)</p>
  </div>
</section>

<section class="section" id="projects" aria-label="projects">
  <div class="container">
    <h3>projects</h3>
    <p>Featured works, links, screenshots, and writeups.</p>
  </div>
</section>

<section class="section" id="contact" aria-label="contact">
  <div class="container">
    <h3>contact</h3>
    <p>How to reach you: email, GitHub, X, etc.</p>
  </div>
</section>

{fallback_note}
<script src="assets/app.js" defer></script>
</body>
</html>
"##,
        media = media_html,
        fallback_note = if has_media { String::new() } else { r#"<p class="desc">No media found. Put a GIF/MP4 under assets/ and set its path in the code.</p>"#.to_string() }
    )
}

fn pick_media_html(tabs: &[Tab]) -> (String, bool) {
    // 1) video_url 優先
    if let Some(t) = tabs.iter().find(|t| !t.video_url.trim().is_empty()) {
        let v = html_attr(t.video_url);
        let html = format!(
            r#"<video playsinline muted loop autoplay preload="metadata" src="{v}"></video>"#
        );
        return (html, true);
    }
    // 2) なければ gif_url
    if let Some(t) = tabs.iter().find(|t| !t.gif_url.trim().is_empty()) {
        let g = html_attr(t.gif_url);
        let html = format!(r#"<img loading="lazy" src="{g}" alt="preview gif">"#);
        return (html, true);
    }
    // 3) プレースホルダ
    (r#"<div class="placeholder"></div>"#.to_string(), false)
}

// ---- helpers ----
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
fn html_attr(s: &str) -> String { html_escape(s).replace('\"', "&quot;") }

// ----------------- Embedded assets -----------------
const STYLE_CSS: &str = r#"
@font-face {
  font-family: 'Mononoki Nerd';
  src: url('fonts/MononokiNerdFont-Regular.ttf') format('truetype');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: 'Mononoki Nerd';
  src: url('fonts/MononokiNerdFont-Bold.ttf') format('truetype');
  font-weight: 700;
  font-style: normal;
  font-display: swap;
}

/* 本文や見出しに適用 */
html { scroll-behavior: smooth; } /* スムーススクロール */
html, body { height: 100%; }
body {
  margin: 0;
  font-family: 'Mononoki Nerd', monospace;   /* ← 統一 */
  font-size: 16px;
  line-height: 1.6;
  color: var(--fg);
  background: linear-gradient(120deg,var(--bg1),var(--bg2)) fixed;
}

/* フォーム/ボタンも継承を確実化 */
button, input, select, textarea { font: inherit; }

/* 見出し・コードも明示しておくと安心 */
h1, h2, h3, h4, h5, h6, code, pre, .headline, .tablink { font-family: 'Mononoki Nerd', monospace; }

:root{--bg1:#0b1220;--bg2:#0b1020;--fg:#e2e8f0;--muted:#94a3b8;--ring:rgba(255,255,255,.1)}
@media (prefers-color-scheme: light){:root{--bg1:#f8fafc;--bg2:#eef2ff;--fg:#0f172a;--muted:#475569;--ring:rgba(0,0,0,.06)}}
*{box-sizing:border-box}
html,body{height:100%}
body{margin:0;font:16px/1.6 system-ui,-apple-system,Segoe UI,Roboto,Ubuntu;color:var(--fg);background:linear-gradient(120deg,var(--bg1),var(--bg2)) fixed}

/* 背景のぼかしオーブ */
.bg-orbs::before,.bg-orbs::after{content:"";position:fixed;inset:auto;filter:blur(60px);z-index:-1;border-radius:9999px}
.bg-orbs::before{top:-60px;left:-40px;width:280px;height:280px;background:rgba(16,185,129,.18)}
.bg-orbs::after{bottom:-80px;right:-60px;width:320px;height:320px;background:rgba(99,102,241,.16)}

/* フルスクリーンの巨大プレビュー */
.preview{
  position:relative;
  width:100vw;
  height:100vh;         /* ブラウザ全面に */
  overflow:hidden;
  background:#082b4b;   /* ローディング時の濃紺 */
}
.media, .media img, .media video{
  position:absolute; inset:0; width:100%; height:100%; object-fit:cover;
}

/* 3行見出し（縦中央寄せ・縦引き伸ばし・適切な行間） */
.headline {
  position:absolute;
  left:1vw;
  top:0;
  bottom:0;
  z-index:3;
  pointer-events:none;

  display:flex;
  flex-direction:column;
  justify-content:center;   /* 縦中央寄せ */

  font-weight:900;
  letter-spacing:-.02em;
  color:#fff;
  text-shadow:0 2px 14px rgba(0,0,0,.55);
  margin:0;
  padding:0;
}
.headline span {
  display:block;
  text-align:left;
  font-size:clamp(28px, 12vh, 22vh);
  line-height:1;             /* 行内は詰める */
  margin:4vh 0;              /* 行間の余白（重なり防止） */
  padding:0;

  transform: scaleY(1.5);    /* 縦方向に引き伸ばす倍率（調整可） */
  transform-origin: left center;
}

/* 下部グラデで可読性UP */
.fade{position:absolute;left:0;right:0;bottom:0;height:120px;background:linear-gradient(0deg,rgba(0,0,0,.45),transparent)}

/* セクション */
.section{
  min-height:100vh;
  display:flex; align-items:center;
  border-top:1px solid var(--ring);
  background:linear-gradient(180deg, transparent, rgba(0,0,0,.04));
}
.container{max-width:1100px; margin:0 auto; padding:6vh 20px;}
.section h3{margin:0 0 12px; font-size:clamp(24px, 5vw, 40px); font-weight:900;}
.section p{margin:0; color:var(--muted)}

/* スクロール時に出現するタブ */
.sticky-tabs{
  position:fixed; top:12px; left:50%; transform:translateX(-50%);
  display:flex; gap:8px;
  background:rgba(15, 23, 42, .55);
  -webkit-backdrop-filter: blur(8px);
  backdrop-filter: blur(8px);
  border:1px solid var(--ring);
  padding:8px;
  border-radius:999px;
  z-index:10;

  opacity:0; pointer-events:none; transition:opacity .25s ease;
}
.sticky-tabs.visible{opacity:1; pointer-events:auto;}
.tablink{
  appearance:none; border:0; border-radius:999px; padding:8px 14px;
  color:#fff; background:transparent; font-weight:800; cursor:pointer;
}
.tablink:hover{background:rgba(255,255,255,.08)}
.tablink:focus{outline:2px solid rgba(255,255,255,.35); outline-offset:2px}

/* 補足文（任意） */
.desc{max-width:1100px;margin:10px auto 32px;padding:0 20px;opacity:.8}

/* プレースホルダ */
.placeholder{
  width:100%; height:100%;
  background:linear-gradient(135deg, rgba(148,163,184,.25), rgba(226,232,240,.35));
}
"#;

const APP_JS: &str = r#"
// スクロール量に応じてタブ表示切替＋クリックでスムーススクロール
(function(){
  const tabs = document.getElementById('stickyTabs');
  const home = document.getElementById('home');

  if (!tabs || !home) return;

  if ('IntersectionObserver' in window) {
    const io = new IntersectionObserver(([entry])=>{
      // home が十分見えている時は隠す、離れたら表示
      const mostlyVisible = entry.intersectionRatio > 0.6;
      tabs.classList.toggle('visible', !mostlyVisible);
    }, { threshold: [0, 0.6, 1] });
    io.observe(home);
  } else {
    // Fallback: スクロール量で判定
    const onScroll = ()=>{
      const y = window.scrollY || document.documentElement.scrollTop;
      tabs.classList.toggle('visible', y > window.innerHeight * 0.4);
    };
    window.addEventListener('scroll', onScroll, { passive:true });
    onScroll();
  }

  // クリックで目的のセクションへスムーススクロール
  tabs.querySelectorAll('.tablink').forEach(btn=>{
    btn.addEventListener('click', ()=>{
      const target = btn.getAttribute('data-target');
      if (!target) return;
      const el = document.querySelector(target);
      if (!el) return;

      try { el.scrollIntoView({ behavior:'smooth', block:'start' }); }
      catch { window.location.hash = target; }
    });
  });
})();
"#;
