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

    let out = Path::new("dist");
    fs::create_dir_all(out)?;
    fs::create_dir_all(out.join("assets"))?;
    fs::create_dir_all(out.join("assets/fonts"))?;
    fs::write(out.join(".nojekyll"), b"")?;
    fs::write(out.join("assets/style.css"), STYLE_CSS)?;
    fs::write(out.join("assets/app.js"), APP_JS)?;
    fs::write(out.join("index.html"), index_page(&tabs, &ver))?;

    println!("\nOK: generated ./dist\nPreview: python3 -m http.server -d dist 8000\n");
    Ok(())
}

fn index_page(tabs: &[Tab], ver: &str) -> String {
    let (media_html, has_media) = pick_media_html(tabs);

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
<script src="assets/app.js?v={v}" defer></script>
</body>
</html>
"##,
        v = ver,
        media = media_html,
        fallback_note = if has_media { String::new() } else { r#"<p class="desc">No media found. Put a GIF/MP4 under assets/ and set its path in the code.</p>"#.to_string() }
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
"#;
