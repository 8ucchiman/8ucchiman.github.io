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
    key: &'a str,         // "robotics" | "3d" | "game" | "music" | "bio" | "others"
    label: &'a str,       // 表示名
    description: &'a str, // 説明（下部テキストに出す）
    gif_url: &'a str,     // .gif（任意）
    video_url: &'a str,   // .mp4/.webm（任意）
}

fn main() -> std::io::Result<()> {
    let tabs = vec![
        Tab { key: "robotics", label: "robotics",   description: "Robotics demos, embedded systems, and real-time CV.", gif_url: "assets/mugen.gif", video_url: "" },
        Tab { key: "3d",       label: "3d render",  description: "Procedural scenes, Blender, OpenGL/GLFW, path tracing.", gif_url: "aasets/samurai_champloo.gif", video_url: "" },
        Tab { key: "game",    label: "game",      description: "Live rigs, DSP experiments, DAW workflows.", gif_url: "", video_url: "" },
        Tab { key: "music",    label: "music",      description: "Live rigs, DSP experiments, DAW workflows.", gif_url: "", video_url: "" },
        Tab { key: "bio",    label: "bio",      description: "Who are you, 8ucchiman?", gif_url: "", video_url: "" },
        Tab { key: "others",   label: "others",     description: "WIP prototypes, notes, utilities, experiments.", gif_url: "", video_url: "" },
    ];

    let out = Path::new("dist");
    fs::create_dir_all(out)?;
    fs::create_dir_all(out.join("assets"))?;
    fs::write(out.join(".nojekyll"), b"")?;
    fs::write(out.join("assets/style.css"), STYLE_CSS)?;
    fs::write(out.join("assets/app.js"), APP_JS)?;

    fs::write(out.join("index.html"), index_page(&tabs))?;

    println!("\nOK: generated ./dist\nPreview: python3 -m http.server -d dist 8000\n");
    Ok(())
}

fn index_page(tabs: &[Tab]) -> String {
    // ヘッダー内タブ（ブランド横）
    let tab_buttons = tabs.iter().map(|t| format!(
        r#"<button class="tab-btn" role="tab" data-tab="{key}" aria-pressed="false">
  <span class="tab-label">{label}</span>
</button>"#,
        key = t.key,
        label = html_escape(t.label)
    )).collect::<Vec<_>>().join("\n");

    // data-* でメディアURLを埋め込む
    let data_map = tabs.iter().map(|t| format!(
        r#"<div class="tab-media" data-tab="{key}" data-gif="{gif}" data-video="{video}"></div>"#,
        key = t.key,
        gif = html_attr(t.gif_url),
        video = html_attr(t.video_url)
    )).collect::<Vec<_>>().join("\n");

    format!(r#"<!doctype html>
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

<header class="site-header">
  <div class="brand">
    <div class="brand-mark">8U</div>
    <div class="brand-text">8ucchiman</div>
  </div>
  <nav class="tabbar" role="tablist">
    {tab_buttons}
  </nav>
</header>

<!-- フルブリード巨大プレビュー（ブラウザ幅いっぱい） -->
<section class="preview">
  <!-- 3行固定の見出し（常時表示） -->
  <h2 class="headline">
    <span>Where are you</span>
    <span>going next,</span>
    <span>8ucchiman?</span>
  </h2>

  <!-- タブで差し替わるメディア -->
  <div class="media" id="media"></div>

  <!-- 現在タブ名 -->
  <div class="badge" id="badge">robotics</div>

  <div class="fade"></div>
</section>

<!-- アクセシブルな説明（任意表示用） -->
<p class="desc" id="desc">Hover a tab to preview.</p>

{data_map}

<script>window.__TABS__ = {json};</script>
<script src="assets/app.js" defer></script>
</body>
</html>
"#,
        tab_buttons = tab_buttons,
        data_map = data_map,
        json = tabs_to_json(tabs),
    )
}

// ---- helpers ----
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
fn html_attr(s: &str) -> String { html_escape(s).replace('\"', "&quot;") }

fn tabs_to_json(tabs: &[Tab]) -> String {
    let mut s = String::from("[");
    for (i, t) in tabs.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(&format!(
            "{{\"key\":\"{}\",\"label\":\"{}\",\"description\":\"{}\",\"gif_url\":\"{}\",\"video_url\":\"{}\"}}",
            t.key, html_attr(t.label), html_attr(t.description), html_attr(t.gif_url), html_attr(t.video_url)
        ));
    }
    s.push(']');
    s
}

// ----------------- Embedded assets -----------------
const STYLE_CSS: &str = r#"
@font-face {
  font-family: 'Mononoki Nerd';
  src: url('assets/fonts/MononokiNerdFont-Bold.woff2') format('woff2');
  font-weight: normal;
  font-style: normal;
}
@font-face {
  font-family: 'Mononoki Nerd';
  src: url('assets/fonts/MononokiNerdFont-Bold.woff2') format('woff2');
  font-weight: bold;
  font-style: normal;
}

/* 本文や見出しに適用 */
body {
  font-family: 'Mononoki Nerd', monospace, system-ui, sans-serif;
}
.headline {
  font-family: 'Mononoki Nerd', monospace;
}
:root{--bg1:#0b1220;--bg2:#0b1020;--fg:#e2e8f0;--muted:#94a3b8;--ring:rgba(255,255,255,.1)}
@media (prefers-color-scheme: light){:root{--bg1:#f8fafc;--bg2:#eef2ff;--fg:#0f172a;--muted:#475569;--ring:rgba(0,0,0,.06)}}
*{box-sizing:border-box}html,body{height:100%}body{margin:0;font:16px/1.6 system-ui,-apple-system,Segoe UI,Roboto,Ubuntu;color:var(--fg);background:linear-gradient(120deg,var(--bg1),var(--bg2)) fixed}
.bg-orbs::before,.bg-orbs::after{content:"";position:fixed;inset:auto;filter:blur(60px);z-index:-1;border-radius:9999px}
.bg-orbs::before{top:-60px;left:-40px;width:280px;height:280px;background:rgba(16,185,129,.18)}
.bg-orbs::after{bottom:-80px;right:-60px;width:320px;height:320px;background:rgba(99,102,241,.16)}

/* ヘッダー（左：ブランド／右：タブ） */
.site-header{display:flex;align-items:center;justify-content:space-between;gap:16px;padding:14px 20px;border-bottom:1px solid var(--ring)}
.brand{display:flex;align-items:center;gap:12px;min-width:180px}
.brand-mark{display:grid;place-items:center;width:36px;height:36px;border-radius:10px;color:#fff;font-weight:900;background:linear-gradient(135deg,#6366f1,#10b981);box-shadow:0 4px 16px rgba(0,0,0,.22)}
.brand-text{font-weight:800;letter-spacing:.2px}
.tabbar{display:flex;flex-wrap:wrap;gap:8px}
.tab-btn{padding:10px 14px;border:0;border-radius:999px;background:transparent;color:var(--fg);font-weight:700;text-transform:capitalize;cursor:pointer}
.tab-btn:hover{background:rgba(255,255,255,.07)}
.tab-btn[aria-pressed="true"]{background:#e2e8f0;color:#0f172a}
@media (prefers-color-scheme: light){.tab-btn[aria-pressed="true"]{background:#0f172a;color:#fff}}

/* フルブリード巨大プレビュー（幅=ブラウザ幅） */
.preview{
  position:relative;
  width:100vw;
  margin-left:calc(50% - 50vw);
  margin-right:calc(50% - 50vw);
  height:60vh;
  min-height:460px;
  overflow:hidden;
  background:#082b4b; /* ローディング時の濃紺 */
}
.media, .media img, .media video{position:absolute;inset:0;width:100%;height:100%;object-fit:cover}

/* 固定3行見出し */
.headline{
  position:absolute; left:1vw; top:12px; z-index:3; pointer-events:none;
  font-weight:900; line-height:1.02;
  font-size:clamp(32px,10vw,120px);
  letter-spacing:-.02em; color:#fff;
  text-shadow:0 2px 14px rgba(0,0,0,.55);
}
.headline span{display:block; text-alighn:left;}

/* 現在タブ名（右下） */
.badge{
  position:absolute; right:16px; bottom:16px; z-index:3;
  background:rgba(0,0,0,.55); color:#fff;
  padding:6px 10px; border-radius:999px; font-size:12px; font-weight:700;
}

/* 下部グラデで可読性UP */
.fade{position:absolute;inset-x:0;bottom:0;height:120px;background:linear-gradient(0deg,rgba(0,0,0,.45),transparent)}

/* 補足文（任意） */
.desc{max-width:1100px;margin:10px auto 32px;padding:0 20px;opacity:.8}
"#;

const APP_JS: &str = r#"
(function(){
  const tabs = (window.__TABS__||[]);
  const tabButtons = Array.from(document.querySelectorAll('.tab-btn'));
  const mediaRoot = document.getElementById('media');
  const badge = document.getElementById('badge');
  const desc = document.getElementById('desc');

  // data-* からメディアURLを取得
  const dataMap = {};
  document.querySelectorAll('.tab-media').forEach(el=>{
    const key = el.getAttribute('data-tab');
    dataMap[key] = {gif: el.getAttribute('data-gif')||'', video: el.getAttribute('data-video')||''};
  });

  const prefersReduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  let currentKey = (tabs[0] && tabs[0].key) || 'robotics';

  function setActive(key, hover){
    currentKey = key;
    tabButtons.forEach(b=> b.setAttribute('aria-pressed', (b.getAttribute('data-tab')===key) ? 'true' : 'false'));
    badge.textContent = key;
    const t = tabs.find(x=>x.key===key);
    if (t) desc.textContent = t.description;

    const {gif, video} = dataMap[key] || {gif:'', video:''};
    mediaRoot.innerHTML = '';
    if (video && !prefersReduced) {
      const v = document.createElement('video');
      v.setAttribute('playsinline',''); v.setAttribute('muted',''); v.setAttribute('loop','');
      v.preload = 'metadata'; v.src = video;
      mediaRoot.appendChild(v);
      if (hover) { v.currentTime = 0; v.play().catch(()=>{}); }
    } else if (gif) {
      const img = document.createElement('img'); img.loading = 'lazy'; img.src = gif; mediaRoot.appendChild(img);
    } else {
      const ph = document.createElement('div');
      ph.style.background = 'linear-gradient(135deg, rgba(148,163,184,.25), rgba(226,232,240,.35))';
      ph.style.width = '100%'; ph.style.height = '100%';
      mediaRoot.appendChild(ph);
    }
  }

  // 初期表示
  setActive(currentKey, false);

  // タブ操作（ホバー/フォーカスでプレビュー）
  tabButtons.forEach(btn=>{
    const key = btn.getAttribute('data-tab');
    btn.addEventListener('mouseenter', ()=> setActive(key, true));
    btn.addEventListener('focus', ()=> setActive(key, true));
    btn.addEventListener('mouseleave', ()=> setActive(key, false));
    btn.addEventListener('blur', ()=> setActive(key, false));
  });

  // ←/→ キーでタブ移動
  window.addEventListener('keydown', (e)=>{
    if (e.key!=='ArrowLeft' && e.key!=='ArrowRight') return;
    e.preventDefault();
    const idx = tabs.findIndex(t=>t.key===currentKey);
    const next = e.key==='ArrowRight' ? (idx+1)%tabs.length : (idx-1+tabs.length)%tabs.length;
    const key = tabs[next].key;
    setActive(key, true);
  });
})();
"#;
