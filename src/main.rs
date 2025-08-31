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
    label: &'a str,       // 表示名（未使用だが将来のため残す）
    description: &'a str, // 説明（未使用だが将来のため残す）
    gif_url: &'a str,     // .gif（任意）
    video_url: &'a str,   // .mp4/.webm（任意）
}

fn main() -> std::io::Result<()> {
    let tabs = vec![
        Tab { key: "robotics", label: "robotics",   description: "Robotics demos, embedded systems, and real-time CV.", gif_url: "assets/mugen.gif", video_url: "" },
        Tab { key: "3d",       label: "3d render",  description: "Procedural scenes, Blender, OpenGL/GLFW, path tracing.", gif_url: "assets/samurai_champloo.gif", video_url: "" },
        Tab { key: "game",     label: "game",       description: "Live rigs, DSP experiments, DAW workflows.", gif_url: "", video_url: "" },
        Tab { key: "music",    label: "music",      description: "Live rigs, DSP experiments, DAW workflows.", gif_url: "", video_url: "" },
        Tab { key: "bio",      label: "bio",        description: "Who are you, 8ucchiman?", gif_url: "", video_url: "" },
        Tab { key: "others",   label: "others",     description: "WIP prototypes, notes, utilities, experiments.", gif_url: "", video_url: "" },
    ];

    let out = Path::new("dist");
    fs::create_dir_all(out)?;
    fs::create_dir_all(out.join("assets"))?;
    fs::write(out.join(".nojekyll"), b"")?;
    fs::write(out.join("assets/style.css"), STYLE_CSS)?;
    // app.js は不要になったので生成しない

    fs::write(out.join("index.html"), index_page(&tabs))?;

    println!("\nOK: generated ./dist\nPreview: python3 -m http.server -d dist 8000\n");
    Ok(())
}

fn index_page(tabs: &[Tab]) -> String {
    // 最初に「video優先、なければgif」のメディアを1つだけ選択
    let (media_html, has_media) = pick_media_html(tabs);

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

<!-- フルスクリーン巨大プレビュー（ブラウザ全面） -->
<section class="preview">
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

{fallback_note}
</body>
</html>
"#,
        media = media_html,
        fallback_note = if has_media { String::new() } else { r#"<p class="desc">No media found. Put a GIF/MP4 under assets/ and set its path in the code.</p>"#.to_string() }
    )
}

fn pick_media_html(tabs: &[Tab]) -> (String, bool) {
    // 1) video_url があるものを優先
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
    // 3) どちらもない場合はプレースホルダ
    let ph = r#"<div class="placeholder"></div>"#.to_string();
    (ph, false)
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


/* 見出し：ブラウザ高さいっぱい、中央寄せで行間ゼロ */
.headline {
  position:absolute;
  left:1vw;
  top:0;
  bottom:0;
  z-index:3;
  pointer-events:none;

  display:flex;
  flex-direction:column;    /* 縦積み */
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
  text-align:left;                /* 左寄せ維持 */
  font-size:clamp(28px, 12vh, 22vh);
  line-height:1;                  /* 行間ゼロ */
  margin:0;
  padding:0;
}



/* 下部グラデで可読性UP */
.fade{position:absolute;left:0;right:0;bottom:0;height:120px;background:linear-gradient(0deg,rgba(0,0,0,.45),transparent)}

/* 補足文（任意） */
.desc{max-width:1100px;margin:10px auto 32px;padding:0 20px;opacity:.8}

/* プレースホルダ */
.placeholder{
  width:100%; height:100%;
  background:linear-gradient(135deg, rgba(148,163,184,.25), rgba(226,232,240,.35));
}
"#;
