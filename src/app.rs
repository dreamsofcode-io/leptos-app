use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-zinc-950 text-zinc-100" style="min-height:100vh">
                <App/>
            </body>
        </html>
    }
}

/// Server-side page visit counter.
#[cfg(feature = "ssr")]
static VISIT_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[server(GetVisitCount, "/api")]
pub async fn get_visit_count() -> Result<u64, ServerFnError> {
    let count = VISIT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    Ok(count)
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-app.css"/>
        <Title text="Leptos App"/>

        <Router>
            <main class="min-h-screen flex justify-center px-5 pt-12 pb-8 sm:px-8 sm:pt-16">
                <div class="w-full max-w-5xl grid gap-7 content-start">
                    <header class="grid gap-5">
                        <div class="max-w-2xl">
                            <span class="inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400">"Leptos Playground"</span>
                            <h1 class="text-5xl sm:text-7xl font-bold tracking-tighter leading-none text-white">"Explore"</h1>
                            <p class="mt-3 max-w-xl text-zinc-400 leading-relaxed">"A full-stack Rust web app built with Leptos."</p>
                        </div>

                        <nav class="flex flex-wrap gap-3" aria-label="Primary">
                            <A href="/" attr:class="inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all">"Counter"</A>
                            <A href="/about" attr:class="inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all">"Router"</A>
                            <A href="/suspense" attr:class="inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all">"Suspense"</A>
                            <A href="/visits" attr:class="inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all">"Visits"</A>
                        </nav>
                    </header>

                    <section class="flex justify-center">
                        <Routes fallback=|| view! {
                            <section class="max-w-lg">
                                <span class="inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400">"404"</span>
                                <h2 class="text-4xl font-bold tracking-tight text-white">"Page not found"</h2>
                                <p class="mt-3 text-zinc-400 leading-relaxed">"The requested route does not exist."</p>
                            </section>
                        }>
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=StaticSegment("about") view=AboutPage />
                            <Route path=StaticSegment("suspense") view=SuspensePage />
                            <Route path=StaticSegment("visits") view=VisitsPage />
                        </Routes>
                    </section>
                </div>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0_i32);

    let count_color = move || {
        if count.get() > 0 {
            "text-green-400"
        } else if count.get() < 0 {
            "text-red-400"
        } else {
            "text-zinc-300"
        }
    };

    view! {
        <section class="grid gap-8 lg:grid-cols-2 lg:items-center">
            <div class="max-w-lg">
                <span class="inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400">"Leptos Demo"</span>
                <h2 class="text-4xl font-bold tracking-tight text-white">"Counter"</h2>
                <p class="mt-3 text-zinc-400 leading-relaxed">"A simple client-side state example in Rust."</p>
            </div>

            <div class="rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-6 shadow-2xl">
                <div class="rounded-2xl border border-white/[0.08] bg-zinc-950/70 p-5">
                    <p class=move || format!("text-center text-7xl font-bold tracking-tighter mb-5 transition-colors {}", count_color())>
                        { move || count.get() }
                    </p>
                    <div class="grid grid-cols-2 gap-3">
                        <button
                            class="rounded-xl border border-white/20 ring-1 ring-white/[0.06] bg-zinc-800 px-4 py-3 font-semibold text-white hover:bg-zinc-700 hover:-translate-y-0.5 transition-all cursor-pointer"
                            on:click=move |_| set_count.update(|c| *c -= 1)
                        >
                            "Decrease"
                        </button>
                        <button
                            class="rounded-xl border border-zinc-300 ring-1 ring-white/20 bg-white px-4 py-3 font-semibold text-zinc-900 hover:bg-zinc-200 hover:-translate-y-0.5 transition-all cursor-pointer"
                            on:click=move |_| set_count.update(|c| *c += 1)
                        >
                            "Increase"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <section class="grid gap-8 lg:grid-cols-2 lg:items-center">
            <div class="max-w-lg">
                <span class="inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400">"Leptos Router"</span>
                <h2 class="text-4xl font-bold tracking-tight text-white">"About"</h2>
                <p class="mt-3 text-zinc-400 leading-relaxed">"This route is handled by leptos_router, so the page content swaps without a full document navigation."</p>
            </div>

            <div class="rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-8 shadow-2xl">
                <p class="text-zinc-300 leading-relaxed">"The layout is shared across routes. Each page renders its own content inside the same shell, demonstrating client-side routing in a full-stack Rust app with server-side rendering and hydration."</p>
            </div>
        </section>
    }
}

#[server(GetDelayedMessage, "/api")]
pub async fn get_delayed_message() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    Ok(String::from("Loaded after a short async delay."))
}

#[component]
fn SuspensePage() -> impl IntoView {
    let data = Resource::new(|| (), |_| get_delayed_message());

    view! {
        <Suspense fallback=move || view! {
            <div class="min-h-[40vh] flex items-center justify-center">
                <div class="w-full max-w-md rounded-3xl border border-white/20 bg-zinc-800/60 backdrop-blur-lg p-8 shadow-2xl shadow-black/40 ring-1 ring-white/[0.06] text-center">
                    <span class="inline-block mb-4 rounded-full bg-amber-500/20 px-4 py-1.5 text-xs font-bold tracking-widest uppercase text-amber-400">
                        "Fetching"
                    </span>
                    <p class="text-zinc-400 leading-relaxed mb-5">"Waiting for async data to resolve..."</p>
                    <div class="flex items-end justify-center gap-1.5 h-8" aria-hidden="true">
                        <span class="loading-bar w-1.5 h-full rounded-full bg-zinc-500"></span>
                        <span class="loading-bar w-1.5 h-full rounded-full bg-zinc-500"></span>
                        <span class="loading-bar w-1.5 h-full rounded-full bg-zinc-500"></span>
                    </div>
                </div>
            </div>
        }>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(message) => view! {
                        <div class="min-h-[40vh] flex items-center justify-center">
                            <div class="w-full max-w-md rounded-3xl border border-white/20 bg-zinc-800/60 backdrop-blur-lg p-8 shadow-2xl shadow-black/40 ring-1 ring-white/[0.06] text-center">
                                <span class="inline-block mb-4 rounded-full bg-green-500/20 px-4 py-1.5 text-xs font-bold tracking-widest uppercase text-green-400">
                                    "Resolved"
                                </span>
                                <p class="text-zinc-300 leading-relaxed">{ message }</p>
                            </div>
                        </div>
                    }.into_any(),
                    Err(e) => view! {
                        <p class="text-red-400">{ format!("Error: {e}") }</p>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn VisitsPage() -> impl IntoView {
    let visits = Resource::new(|| (), |_| get_visit_count());

    view! {
        <section class="grid gap-8 lg:grid-cols-2 lg:items-center">
            <div class="max-w-lg">
                <span class="inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400">"Server Function"</span>
                <h2 class="text-4xl font-bold tracking-tight text-white">"Page Visits"</h2>
                <p class="mt-3 text-zinc-400 leading-relaxed">"This counter lives on the server. Each load calls a server function that increments and returns the visit count."</p>
            </div>

            <div class="rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-6 shadow-2xl">
                <div class="rounded-2xl border border-white/[0.08] bg-zinc-950/70 p-5">
                    <Suspense fallback=move || view! {
                        <p class="text-center text-7xl font-bold tracking-tighter text-zinc-500 mb-5">"..."</p>
                    }>
                        {move || Suspend::new(async move {
                            match visits.await {
                                Ok(count) => view! {
                                    <p class="text-center text-7xl font-bold tracking-tighter text-green-400 mb-5">{ count }</p>
                                }.into_any(),
                                Err(e) => view! {
                                    <p class="text-center text-red-400">{ format!("Error: {e}") }</p>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                    <p class="text-center text-zinc-400 text-sm">"Total visits (server-side counter)"</p>
                </div>
            </div>
        </section>
    }
}
