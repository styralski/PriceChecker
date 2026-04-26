pub const APP_STYLES: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,700&family=Space+Grotesk:wght@400;500;700&display=swap');

:root {
    --bg-a: #f4f8f2;
    --bg-b: #e7f3ea;
    --ink: #10231a;
    --muted: #476054;
    --accent: #1c7a46;
    --accent-deep: #0f5a34;
    --card: rgba(255, 255, 255, 0.84);
    --line: rgba(17, 60, 38, 0.12);
    --warn-bg: #fff7e9;
    --warn-ink: #7e4d00;
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    font-family: 'Space Grotesk', sans-serif;
    color: var(--ink);
    background:
        radial-gradient(circle at 15% 18%, rgba(28, 122, 70, 0.16) 0%, transparent 34%),
        radial-gradient(circle at 90% 8%, rgba(35, 135, 92, 0.18) 0%, transparent 30%),
        linear-gradient(160deg, var(--bg-a) 0%, var(--bg-b) 100%);
    min-height: 100vh;
}

.pc-page {
    max-width: 1120px;
    margin: 0 auto;
    padding: 1.2rem 1rem 3rem;
    position: relative;
}

.topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.65rem 0.8rem;
    border: 1px solid var(--line);
    border-radius: 16px;
    backdrop-filter: blur(6px);
    background: rgba(255, 255, 255, 0.72);
}

.brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-decoration: none;
    color: inherit;
}

.brand-mark {
    width: 42px;
    height: 42px;
    border-radius: 11px;
    border: 2px solid var(--accent);
    display: grid;
    place-items: center;
    font-weight: 700;
    letter-spacing: 0.02em;
    color: var(--accent-deep);
    background: white;
}

.brand-text {
    font-family: 'Fraunces', serif;
    font-size: clamp(1.1rem, 2vw, 1.4rem);
    letter-spacing: 0.01em;
}

.desktop-nav {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.desktop-nav a,
.mobile-nav a {
    text-decoration: none;
    color: var(--ink);
    font-weight: 600;
    opacity: 0.88;
}

.desktop-nav a:hover,
.mobile-nav a:hover {
    opacity: 1;
    color: var(--accent-deep);
}

.menu-btn {
    display: none;
    border: 1px solid var(--line);
    background: white;
    color: var(--ink);
    font-weight: 600;
    border-radius: 999px;
    padding: 0.42rem 0.9rem;
    cursor: pointer;
}

.mobile-nav {
    margin-top: 0.7rem;
    border: 1px solid var(--line);
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.92);
    padding: 0.6rem 0.8rem;
    display: flex;
    gap: 1rem;
}

.hero {
    margin-top: 1.2rem;
    border: 1px solid var(--line);
    border-radius: 20px;
    padding: clamp(1rem, 2.8vw, 2rem);
    background: linear-gradient(128deg, rgba(255, 255, 255, 0.95), rgba(233, 248, 238, 0.95));
}

.hero h1 {
    margin: 0;
    font-family: 'Fraunces', serif;
    font-size: clamp(1.7rem, 4.2vw, 3rem);
    line-height: 1.08;
    max-width: 18ch;
}

.hero p {
    margin: 0.9rem 0 0;
    color: var(--muted);
    font-size: clamp(0.98rem, 2vw, 1.1rem);
    max-width: 64ch;
}

.panel {
    margin-top: 1rem;
    border: 1px solid var(--line);
    border-radius: 20px;
    background: var(--card);
    padding: 1rem;
    backdrop-filter: blur(5px);
}

.about-content {
    margin-top: 1rem;
    border: 1px solid var(--line);
    border-radius: 20px;
    background: var(--card);
    padding: clamp(1rem, 2.5vw, 1.6rem);
    backdrop-filter: blur(5px);
}

.about-content p {
    margin: 0.65rem 0;
    color: var(--muted);
    line-height: 1.5;
}

.about-header {
    font-family: 'Fraunces', serif;
    margin: 1rem 0 0.35rem;
    font-size: clamp(1.3rem, 3vw, 2rem);
    color: var(--ink);
}

.search-form {
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
}

.search-form label {
    font-weight: 700;
}

.input-row {
    display: flex;
    gap: 0.55rem;
}

.input-row input {
    flex: 1;
    min-width: 0;
    border: 1px solid #bfd2c3;
    border-radius: 12px;
    background: white;
    padding: 0.78rem 0.95rem;
    font-size: 0.98rem;
    outline: none;
}

.input-row input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 4px rgba(28, 122, 70, 0.12);
}

.input-row button {
    border: 0;
    border-radius: 12px;
    background: linear-gradient(135deg, var(--accent), var(--accent-deep));
    color: white;
    font-weight: 700;
    letter-spacing: 0.01em;
    padding: 0 1rem;
    cursor: pointer;
}

.input-row button[disabled] {
    opacity: 0.6;
    cursor: wait;
}

.hint {
    margin: 0;
    color: #5d7064;
    font-size: 0.92rem;
}

.suggestion-wrap {
    display: flex;
    flex-direction: column;
    gap: 0.45rem;
}

.suggestion-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
}

.suggestion-chip {
    border: 1px solid #b7d2c0;
    border-radius: 999px;
    background: #f1fbf5;
    color: #1f5a3c;
    font-weight: 600;
    font-size: 0.86rem;
    padding: 0.34rem 0.72rem;
    cursor: pointer;
}

.suggestion-chip:hover {
    background: #e7f8ee;
    border-color: #9ec6af;
}

.suggestion-chip:focus-visible {
    outline: 2px solid rgba(28, 122, 70, 0.45);
    outline-offset: 2px;
}

.status {
    margin-top: 0.8rem;
    padding: 0.65rem 0.8rem;
    border-radius: 11px;
    font-weight: 500;
}

.status-warn {
    background: var(--warn-bg);
    color: var(--warn-ink);
    border: 1px solid #f1d7a8;
}

.results-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.8rem;
    margin-bottom: 0.75rem;
}

.results-head h2 {
    margin: 0;
    font-family: 'Fraunces', serif;
    font-size: clamp(1.2rem, 3vw, 1.8rem);
}

.tag {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    border-radius: 999px;
    font-size: 0.85rem;
    font-weight: 700;
    padding: 0.34rem 0.7rem;
}

.tag-cache {
    color: #145f36;
    background: #e7f8ee;
    border: 1px solid #bde6cd;
}

.tag-fresh {
    color: #0b4e70;
    background: #eaf7ff;
    border: 1px solid #bee6fb;
}

.offer-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.8rem;
}

.offer-card {
    border: 1px solid var(--line);
    border-radius: 14px;
    background: white;
    padding: 0.9rem;
    display: flex;
    flex-direction: column;
    gap: 0.7rem;
}

.offer-card h3 {
    margin: 0;
    line-height: 1.25;
    font-size: 1rem;
}

.meta-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.6rem;
}

.shop {
    color: #4f6658;
    font-weight: 500;
}

.price {
    font-size: 1.02rem;
    font-weight: 800;
    color: #0d5f36;
    white-space: nowrap;
}

.open-link {
    width: fit-content;
    text-decoration: none;
    color: white;
    font-weight: 700;
    background: #174f33;
    border-radius: 10px;
    padding: 0.4rem 0.7rem;
}

.open-link:hover {
    background: #103f27;
}

.time {
    margin: 0;
    color: #6f8276;
    font-size: 0.82rem;
}

.empty {
    border: 1px dashed #b6ccb9;
    border-radius: 14px;
    padding: 1rem;
    color: #567060;
    background: #f9fffb;
}

.skeleton-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.8rem;
}

.skeleton-card {
    height: 150px;
    border-radius: 14px;
    border: 1px solid var(--line);
    background: linear-gradient(
        110deg,
        rgba(245, 250, 246, 0.8) 8%,
        rgba(232, 242, 235, 0.95) 18%,
        rgba(245, 250, 246, 0.8) 33%
    );
    background-size: 200% 100%;
    animation: shimmer 1.2s linear infinite;
}

.db-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.8rem;
    flex-wrap: wrap;
    margin: 0.7rem 0 1rem;
}

.db-search-form {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 280px;
}

.search-icon {
    width: 36px;
    height: 36px;
    border: 1px solid var(--line);
    border-radius: 10px;
    background: white;
    display: grid;
    place-items: center;
    color: #2b5f45;
}

.db-search-form input {
    flex: 1;
    min-width: 0;
    border: 1px solid #bfd2c3;
    border-radius: 12px;
    background: white;
    padding: 0.72rem 0.9rem;
    font-size: 0.95rem;
    outline: none;
}

.db-search-form input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 4px rgba(28, 122, 70, 0.12);
}

.db-search-form button {
    border: 0;
    border-radius: 12px;
    background: linear-gradient(135deg, var(--accent), var(--accent-deep));
    color: white;
    font-weight: 700;
    letter-spacing: 0.01em;
    padding: 0.72rem 1rem;
    cursor: pointer;
}

.db-search-form button[disabled] {
    opacity: 0.6;
    cursor: wait;
}

.db-date-filter {
    display: flex;
    align-items: center;
    gap: 0.45rem;
}

.db-date-filter label {
    color: #3f5f50;
    font-weight: 600;
    font-size: 0.88rem;
}

.db-date-filter input {
    border: 1px solid #bfd2c3;
    border-radius: 10px;
    background: white;
    color: var(--ink);
    font-size: 0.9rem;
    padding: 0.55rem 0.65rem;
    outline: none;
}

.db-date-filter input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 4px rgba(28, 122, 70, 0.12);
}

.sort-buttons {
    display: inline-flex;
    gap: 0.45rem;
}

.sort-btn {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    border: 1px solid var(--line);
    background: white;
    color: #2b5f45;
    display: grid;
    place-items: center;
    cursor: pointer;
}

.sort-btn.active {
    color: white;
    border-color: transparent;
    background: linear-gradient(135deg, var(--accent), var(--accent-deep));
}

.table-wrap {
    overflow-x: auto;
}

.offers-table {
    width: 100%;
    min-width: 880px;
    border-collapse: collapse;
    border: 1px solid var(--line);
    border-radius: 12px;
    overflow: hidden;
    background: rgba(255, 255, 255, 0.92);
}

.offers-table th,
.offers-table td {
    text-align: left;
    vertical-align: top;
    padding: 0.72rem 0.82rem;
    border-bottom: 1px solid rgba(17, 60, 38, 0.08);
}

.offers-table th {
    background: #eef7f1;
    color: #315846;
    font-size: 0.84rem;
    text-transform: uppercase;
    letter-spacing: 0.02em;
}

.offers-table tbody tr:hover {
    background: #f6fcf8;
}

.reveal {
    opacity: 0;
    transform: translateY(12px);
    animation: reveal 600ms ease forwards;
}

.delay-1 {
    animation-delay: 80ms;
}

.delay-2 {
    animation-delay: 160ms;
}

.delay-3 {
    animation-delay: 240ms;
}

@keyframes reveal {
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes shimmer {
    to {
        background-position-x: -200%;
    }
}

@media (max-width: 900px) {
    .offer-grid,
    .skeleton-grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 700px) {
    .desktop-nav {
        display: none;
    }

    .menu-btn {
        display: inline-flex;
    }

    .input-row {
        flex-direction: column;
    }

    .input-row button {
        min-height: 42px;
    }

    .offer-grid,
    .skeleton-grid {
        grid-template-columns: 1fr;
    }

    .db-search-form {
        min-width: 100%;
    }

    .db-date-filter {
        width: 100%;
        justify-content: space-between;
    }

    .db-date-filter input {
        flex: 1;
    }
}
"#;
