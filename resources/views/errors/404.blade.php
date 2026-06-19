<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="robots" content="noindex">
        <title>Page not found - ChromaVale</title>
        <link rel="icon" href="/favicon.svg" type="image/svg+xml">
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Instrument+Sans:wght@400..700&display=swap" rel="stylesheet">
        <style>
            * { margin: 0; padding: 0; box-sizing: border-box; }
            body {
                min-height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: #ffffff;
                color: #16161a;
                font-family: 'Instrument Sans', ui-sans-serif, system-ui, sans-serif;
                -webkit-font-smoothing: antialiased;
                text-align: center;
                padding: 24px;
            }
            .wrap { max-width: 30rem; }
            .mark { width: 56px; height: 56px; margin: 0 auto 2rem; }
            .code {
                font-size: 0.78rem;
                font-weight: 700;
                text-transform: uppercase;
                letter-spacing: 0.14em;
                color: #71717a;
                margin-bottom: 0.9rem;
            }
            h1 {
                font-size: clamp(1.8rem, 5vw, 2.4rem);
                line-height: 1.1;
                letter-spacing: -0.03em;
                font-weight: 600;
            }
            p {
                margin-top: 1rem;
                font-size: 1.05rem;
                line-height: 1.6;
                color: #52525b;
            }
            .btn {
                display: inline-flex;
                align-items: center;
                gap: 0.5rem;
                margin-top: 2rem;
                padding: 0.9rem 1.4rem;
                border-radius: 14px;
                background: #16161a;
                color: #fff;
                font-weight: 600;
                font-size: 1rem;
                text-decoration: none;
                box-shadow: 0 5px 0 0 #000;
                transition: transform 0.11s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.11s, background 0.15s;
            }
            .btn:hover { background: #26262e; }
            .btn:active { transform: translateY(5px); box-shadow: 0 0 0 0 #000; }
            .bar {
                height: 6px;
                width: 160px;
                margin: 2.5rem auto 0;
                border-radius: 999px;
                background: linear-gradient(90deg, #ef6f5e, #f2b04a 26%, #5bd6a0 52%, #4da6ff 76%, #9b6be5);
            }
            @media (prefers-color-scheme: dark) {
                body { background: #0c0c0e; color: #fafafa; }
                p { color: #a1a1aa; }
                .btn { background: #fff; color: #16161a; box-shadow: 0 5px 0 0 rgba(0,0,0,0.6); }
                .btn:hover { background: #f0f0f2; }
            }
        </style>
    </head>
    <body>
        <div class="wrap">
            <svg class="mark" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <circle cx="16" cy="16" r="13.5" stroke="currentColor" stroke-width="2.5" />
                <circle cx="16" cy="16" r="6.5" fill="url(#p404)" />
                <defs>
                    <linearGradient id="p404" x1="9" y1="9" x2="23" y2="23" gradientUnits="userSpaceOnUse">
                        <stop stop-color="#FF7A6B" />
                        <stop offset="0.3" stop-color="#FFC24B" />
                        <stop offset="0.55" stop-color="#5BD6A0" />
                        <stop offset="0.78" stop-color="#4DA6FF" />
                        <stop offset="1" stop-color="#9B6BE5" />
                    </linearGradient>
                </defs>
            </svg>
            <div class="code">Error 404</div>
            <h1>This shade isn't on the palette.</h1>
            <p>The page you're looking for doesn't exist or has moved. Let's get you back to a color you know.</p>
            <a class="btn" href="/">
                <svg width="18" height="18" viewBox="0 0 20 20" fill="none" aria-hidden="true">
                    <path d="M12 5l-5 5 5 5" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                Back to home
            </a>
            <div class="bar"></div>
        </div>
    </body>
</html>
