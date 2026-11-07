import { chromium } from "playwright";
import { readdir, mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

const BASE = process.env.QA_BASE_URL ?? "http://localhost:4321";
const OUT = ".qa/screenshots";

async function walk(dir) {
        const entries = await readdir(dir, { withFileTypes: true });
        const files = [];
        for (const e of entries) {
                const p = path.join(dir, e.name);
                if (e.isDirectory()) files.push(...(await walk(p)));
                else if (e.name === "index.html") files.push(p);
        }
        return files;
}

async function routes() {
        const files = await walk("dist");
        return files.map((f) => {
                const rel = path.relative("dist", path.dirname(f));
                return "/" + (rel === "." || rel === "" ? "" : rel + "/");
        });
}

const VIEWPORTS = {
        desktop: { width: 1440, height: 900 },
        mobile: { width: 390, height: 844 },
};

const failures = [];
const fail = (route, viewport, theme, kind, detail) =>
        failures.push({ route, viewport, theme, kind, detail });

const browser = await chromium.launch();
const all = await routes();
await mkdir(OUT, { recursive: true });

for (const route of all) {
        for (const [vpName, viewport] of Object.entries(VIEWPORTS)) {
                for (const theme of ["light", "dark"]) {
                        const ctx = await browser.newContext({
                                viewport,
                                reducedMotion: "reduce",
                        });
                        const page = await ctx.newPage();

                        page.on("console", (m) => {
                                if (m.type() === "error")
                                        fail(route, vpName, theme, "console", m.text());
                        });
                        page.on("pageerror", (e) =>
                                fail(route, vpName, theme, "pageerror", e.message),
                        );
                        page.on("response", (r) => {
                                if (r.status() >= 400)
                                        fail(route, vpName, theme, "http", `${r.status()} ${r.url()}`);
                        });

                        await page.addInitScript((t) => {
                                try {
                                        localStorage.setItem("starlight-theme", t);
                                } catch {}
                        }, theme);

                        await page.goto(BASE + route, { waitUntil: "networkidle" });
                        await page.evaluate(() => document.fonts.ready);

                        const applied = await page.evaluate(
                                () => document.documentElement.dataset.theme,
                        );
                        if (applied !== theme) {
                                const toggle = page
                                        .locator("starlight-theme-select button, button[data-theme-toggle]")
                                        .first();
                                if (await toggle.count()) {
                                        await toggle.click();
                                        await page.waitForTimeout(150);
                                }
                        }

                        const probe = await page.evaluate(() => {
                                const main = document.querySelector("main");
                                const imgs = [...document.querySelectorAll("img")];
                                const logo = document.querySelector(".site-title img, header img");
                                return {
                                        theme: document.documentElement.dataset.theme,
                                        h1: document.querySelectorAll("h1").length,
                                        textLen: (main?.innerText ?? "").trim().length,
                                        overflow:
                                                document.documentElement.scrollWidth > window.innerWidth + 1,
                                        brokenImgs: imgs
                                                .filter((i) => !i.complete || i.naturalWidth === 0)
                                                .map((i) => i.currentSrc || i.src),
                                        logo: logo
                                                ? {
                                                                src: logo.currentSrc || logo.src,
                                                                naturalW: logo.naturalWidth,
                                                                naturalH: logo.naturalHeight,
                                                                renderW: logo.clientWidth,
                                                                renderH: logo.clientHeight,
                                                        }
                                                : null,
                                };
                        });

                        if (probe.theme !== theme)
                                fail(route, vpName, theme, "theme", `applied=${probe.theme}`);
                        if (probe.h1 !== 1) fail(route, vpName, theme, "h1", `count=${probe.h1}`);
                        if (probe.textLen < 200)
                                fail(route, vpName, theme, "empty", `text=${probe.textLen}`);
                        if (probe.overflow)
                                fail(route, vpName, theme, "overflow", "horizontal scrollbar");
                        for (const src of probe.brokenImgs)
                                fail(route, vpName, theme, "image", src);

                        if (probe.logo) {
                                if (!/\.png(\?|$)/i.test(probe.logo.src))
                                        fail(route, vpName, theme, "logo-format", probe.logo.src);
                                if (
                                        probe.logo.renderW > probe.logo.naturalW + 1 ||
                                        probe.logo.renderH > probe.logo.naturalH + 1
                                )
                                        fail(
                                                route,
                                                vpName,
                                                theme,
                                                "logo-upscale",
                                                `render ${probe.logo.renderW}x${probe.logo.renderH} > natural ${probe.logo.naturalW}x${probe.logo.naturalH}`,
                                        );
                        }

                        const slug =
                                route === "/"
                                        ? "index"
                                        : route.replace(/^\/|\/$/g, "").replace(/\//g, "_");
                        await page.screenshot({
                                path: path.join(OUT, `${slug}--${vpName}--${theme}.png`),
                                fullPage: true,
                        });
                        await ctx.close();
                }
        }
}

await browser.close();

// Screenshot dist/404.html once (desktop/dark) outside the route matrix:
// the 404 route legitimately returns HTTP 404, so response-status checks do
// not apply, and it is excluded from the eyeball pass.
const browser2 = await chromium.launch();
{
        const ctx = await browser2.newContext({
                viewport: VIEWPORTS.desktop,
                reducedMotion: "reduce",
        });
        const page = await ctx.newPage();
        await page.addInitScript((t) => {
                try {
                        localStorage.setItem("starlight-theme", t);
                } catch {}
        }, "dark");
        await page.goto(BASE + "/404", { waitUntil: "networkidle" });
        await page.evaluate(() => document.fonts.ready);
        await page.screenshot({
                path: path.join(OUT, "404--desktop--dark.png"),
                fullPage: true,
        });
        await ctx.close();
}
await browser2.close();

await writeFile(".qa/failures.json", JSON.stringify(failures, null, 2));

if (failures.length) {
        console.error(`QA FAILED — ${failures.length} issue(s):`);
        for (const f of failures)
                console.error(
                        `  [${f.kind}] ${f.route} (${f.viewport}/${f.theme}): ${f.detail}`,
                );
        process.exit(1);
}
console.log(`QA PASSED — ${all.length} routes × 2 viewports × 2 themes`);
