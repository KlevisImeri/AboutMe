use crate::components::{Content, DataTable};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};

#[component]
pub fn ResumePage(locale: String) -> Element {
    let language = locale.parse().expect("supported CV locale");
    use_init_i18n(move || {
        I18nConfig::new(language)
            .with_locale((
                "en".parse().expect("valid English locale"),
                include_str!("../../locales/en.ftl"),
            ))
            .with_locale((
                "sq".parse().expect("valid Albanian locale"),
                include_str!("../../locales/sq.ftl"),
            ))
    });
    rsx! { ResumeContent { locale } }
}

#[component]
fn ResumeContent(locale: String) -> Element {
    rsx! {
        Content {
            ContactHeader {}
            div { class: "cv-details", lang: locale,
                CvSection { id: "introduction", Introduction {} }
                CvSection { id: "personal-data", PersonalData {} }
                CvSection { id: "education", Education {} }
                CvSection { id: "research", Research {} }
                CvSection { id: "experience", Experience {} }
                CvSection { id: "projects", Projects {} }
                CvSection { id: "open-source", OpenSource {} }
                CvSection { id: "awards", Awards {} }
                CvSection { id: "skills", Skills {} }
                CvSection { id: "language-skills", Languages {} }
                hr {}
            }
        }
    }
}

#[component]
fn Introduction() -> Element {
    rsx! {
        p {
            {t!("intro-greeting")} " " {t!("intro-description")}
        }
    }
}

#[component]
fn CvSection(id: &'static str, children: Element) -> Element {
    let title = t!(id);
    rsx! {
        details { class: "cv-section", open: true,
            summary { class: "cv-section-heading",
                hr {}
                h4 { "[{title}]" }
            }
            {children}
        }
    }
}

#[component]
fn CvEntry(heading: Element, children: Element, #[props(default = true)] shaded: bool) -> Element {
    let heading_class = if shaded {
        "cv-entry-heading experience-heading"
    } else {
        "cv-entry-heading"
    };
    rsx! {
        details { class: "cv-entry", open: true,
            summary { class: heading_class,
                {heading}
            }
            {children}
        }
    }
}

#[component]
fn CvTableRow(label: String, children: Element, #[props(default = 1)] columns: usize) -> Element {
    let mut expanded = use_signal(|| true);
    rsx! {
        tr { class: if expanded() { "" } else { "cv-item-collapsed" },
            td {
                button {
                    class: "cv-item-toggle",
                    r#type: "button",
                    "aria-label": label,
                    "aria-expanded": expanded(),
                    onclick: move |_| expanded.toggle(),
                    "{label}"
                }
            }
            if expanded() {
                {children}
            } else {
                td { colspan: columns }
            }
        }
    }
}

#[component]
fn ContactHeader() -> Element {
    let website = t!("website-url");
    rsx! {
        header { class: "relative mx-auto text-center",
            div { class: "inline-block text-center",
                h1 { class: "contact-name mb-[0.5em] text-2xl font-bold", "Klevis Imeri" }
                div { a { href: website.clone(), "{website}" } }
                div { class: "mt-[0.5em] text-base",
                    span { class: "contact-item", "klevisimeri11@gmail.com" }
                    span { class: "separator mx-[0.7em] text-[#ccc]", "|" }
                    span { class: "contact-item", "+383 44 165 634" }
                    span { class: "separator mx-[0.7em] text-[#ccc]", "|" }
                    a { href: "https://github.com/KlevisImeri/", class: "contact-item contact-github", "github.com/KlevisImeri" }
                }
            }
        }
    }
}

#[component]
fn PersonalData() -> Element {
    let mut expanded = use_signal(|| [true; 4]);
    let fields = [
        (t!("date-of-birth"), "08.05.2004".to_string()),
        (t!("place-of-birth"), t!("birthplace")),
        (t!("nationality"), t!("citizenship")),
        (t!("marital-status"), t!("single")),
    ];
    rsx! {
        div { class: "personal-data",
        DataTable {
            thead { tr {
                for (index, (label, _)) in fields.iter().enumerate() {
                    th { class: if expanded()[index] { "" } else { "cv-item-collapsed" },
                        button {
                            class: "cv-item-toggle",
                            r#type: "button",
                            "aria-label": label.clone(),
                            "aria-expanded": expanded()[index],
                            onclick: move |_| {
                                let value = expanded()[index];
                                expanded.write()[index] = !value;
                            },
                            "{label}"
                        }
                    }
                }
            } }
            tbody { tr {
                for (index, (_, value)) in fields.iter().enumerate() {
                    td { class: if expanded()[index] { "" } else { "cv-item-collapsed" },
                        if expanded()[index] { "{value}" }
                    }
                }
            } }
        }
        }
    }
}

#[component]
fn Education() -> Element {
    rsx! {
        div { class: "education-entries",
            CvEntry {
                heading: rsx! { strong { {t!("eth-name")} } " | " {t!("eth-period")} " | "
                    a { href: "https://ethz.ch/en.html", {t!("view-website")} }
                },
                p { {t!("eth-degree")} }
            }
            CvEntry {
                heading: rsx! { strong { {t!("bme-name")} } " | " {t!("bme-period")} " | "
                    a { href: "https://www.bme.hu/en", {t!("view-website")} }
                },
                p { {t!("bme-degree")} }
                p { class: "education-meta",
                    {t!("bme-honours")} " · " {t!("gpa", value: "4.97/5.00")} " · " {t!("past-scholarship")}
                }
            }
            CvEntry {
                heading: rsx! { strong { {t!("school-name")} } " | " {t!("school-period")} " | "
                    a { href: "https://mileniumi3.net/", {t!("view-website")} }
                },
                p { {t!("school-degree")} }
                p { class: "education-meta",
                    {t!("school-honours")} " · " {t!("gpa", value: "5.00/5.00")}
                }
            }
        }
    }
}

#[component]
fn Research() -> Element {
    rsx! {
        CvEntry {
        heading: rsx! {
            strong { {t!("tdk-title")} } " | 2025 | "
            a { href: "https://tdk.bme.hu/conference/VIK/2025/sessions/sw/paper/Modellellenorzesi-portfoliok-hatekonysaganak", {t!("view-paper")} }
        },
        p {
            em { {t!("tdk-faculty")} } br {}
            {t!("tdk-paper")} br {}
            em { {t!("tdk-section")} }
        }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("thesis-title")} } " | 2025 | "
            a { href: "https://aboutme.klevis.xyz/thesis.pdf", {t!("view-thesis")} }
        },
        p {
            {t!("thesis-description")}
        }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("witness-title")} } " | 2025 | "
            a { href: "https://aboutme.klevis.xyz/witness-theta-paper.pdf", {t!("view-report")} }
        },
        p {
            {t!("research-group")} br {}
            {t!("witness-description")}
        }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("publication-title")} } " | 2025 | "
            a { href: "https://link.springer.com/chapter/10.1007/978-3-031-91572-7_16", {t!("view-chapter")} }
        },
        p {
            em { "Automatic Reconstruction of Ancient Colored Glass Windows" } br {}
            {t!("publication-description")} br {}
            {t!("publication-publisher", publisher: "Springer Nature")}
        }
        }
    }
}

// Fluent can reorder named placeholders without embedding HTML in translations.
// Only these application-owned markers become links; other text stays plain text.
#[component]
fn LinkedText(text: String) -> Element {
    let links = [
        ("theta", "Theta", "https://github.com/ftsrg/theta"),
        ("ftsrg", "FTSRG, BME", "https://ftsrg.mit.bme.hu/hu/"),
        ("supervisor", "Zsófia Ádám", "https://adamzsofi.github.io/"),
        (
            "witness",
            "Software Witness 2.0",
            "https://www.sosy-lab.org/research/pub/2024-SPIN.Software_Verification_Witnesses_2.0.pdf",
        ),
        (
            "benchcloud",
            "Theta BenchCloud",
            "http://benchcloud.mit.bme.hu:8088/benchcloud/webclient/master/info",
        ),
        (
            "benchmarks",
            "SV-Benchmarks",
            "https://gitlab.com/sosy-lab/benchmarking/sv-benchmarks",
        ),
        (
            "benchexec",
            "BenchExec",
            "https://github.com/sosy-lab/benchexec",
        ),
        ("vprog", "Vprog", "https://vprog.hu/"),
        ("icpc", "ICPC Hungary", "https://cs.bme.hu/icpc/results/"),
        ("cerc", "CERC", "https://cerc.acm.si/"),
    ];
    rsx! {
        for (index, part) in text.split('§').enumerate() {
            if index % 2 == 1 {
                if let Some((_, label, href)) = links.iter().find(|(id, _, _)| *id == part) {
                    a { href: *href, "{label}" }
                } else {
                    "{part}"
                }
            } else {
                "{part}"
            }
        }
    }
}

#[component]
fn Experience() -> Element {
    rsx! {
        CvEntry {
            heading: rsx! {
                strong { {t!("dentalhealth-title")} } " | " {t!("dentalhealth-period")} " | DentalHealth | "
                a { href: "https://dentalhealth.al", {t!("view-website")} }
            },
            p { {t!("dentalhealth-development")} }
            p { {t!("dentalhealth-operations")} }
        }
        CvEntry {
            heading: rsx! {
                strong { {t!("webvalto-title")} } " | " {t!("webvalto-period")} " | Webváltó | "
                a { href: "https://www.webvalto.hu/en", {t!("view-website")} }
            },
            p { {t!("webvalto-description")} }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("researcher-title")} } " | " {t!("researcher-period")} " | BME, Budapest"
        },
        p {
            LinkedText { text: t!("researcher-description", software: "§theta§", group: "§ftsrg§", supervisor: "§supervisor§", witness: "§witness§", infrastructure: "§benchcloud§", suite: "§benchmarks§", tool: "§benchexec§") }
        }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("instructor-title")} }
            " | " {t!("instructor-period")} " | Budapest | "
            a { href: "https://www.db.bme.hu/databases/", {t!("view-website")} }
        },
        p { {t!("instructor-description")} }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("engineer-title")} } " | " {t!("engineer-period")} " | " {t!("engineer-location")}
        },
        p { {t!("engineer-description")} }
        }
        CvEntry {
        heading: rsx! {
            strong { {t!("competitive-title")} } " | " {t!("competitive-period")} " | Budapest"
        },
        p {
            LinkedText { text: t!("competitive-description", group: "§vprog§", practice: "Vprog Advance", contest: "§icpc§", regional: "§cerc§") }
        }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        CvEntry {
        heading: rsx! {
            strong { "SimFN " {t!("and")} " Integrator" }
            " | Java, C/C++ | "
            a { href: "https://github.com/KlevisImeri/SimFN", {t!("view-project", name: "SimFN")} }
            " | "
            a { href: "https://github.com/KlevisImeri/Integrator", {t!("view-project", name: "Integrator (C)")} }
            " | "
            a { href: "https://github.com/KlevisImeri/Integrator2", {t!("view-project", name: "Integrator (C++)")} }
        },
        p { {t!("simfn-description")} br {} {t!("integrator-description")} }
        }
        CvEntry {
            heading: rsx! {
                strong { "drag" }
                " | C, X11, Wayland | "
                a { href: "https://github.com/KlevisImeri/drag", {t!("view-project", name: "drag")} }
            },
            p { {t!("drag-description")} }
        }
        CvEntry {
            heading: rsx! {
                strong { "git-sync-submodule-refs" }
                " | Rust, Lean, Charon, Aeneas | "
                a { href: "https://github.com/KlevisImeri/git-sync-submodule-refs", {t!("view-project", name: "git-sync-submodule-refs")} }
            },
            p { {t!("git-sync-description")} }
            p { em { {t!("git-sync-disclaimer")} } }
        }
        CvEntry {
        heading: rsx! {
            strong { "BlochSphere" }
            " | Svelte, Bun, Vite.js, Three.js | "
            a { href: "https://klevisimeri.github.io/BlochSphere/", {t!("view-project", name: "BlochSphere")} }
        },
        p {
            {t!("bloch-description")} br {}
            {t!("bloch-features")} br {}
            {t!("bloch-tools")}
        }
        }
    }
}

#[component]
fn OpenSource() -> Element {
    rsx! {
        CvEntry {
            heading: rsx! {
                strong { "compile-mode.nvim" }
                " | Lua | "
                a { href: "https://github.com/ej-shafran/compile-mode.nvim/pull/140", "PR #140" }
            },
            p { {t!("compile-mode-description")} }
        }
        CvEntry {
            heading: rsx! {
                strong { "VProg Discord Bot" }
                " | Rust | "
                a { href: "https://github.com/bmevprog/vbot/pull/1", "PR #1" }
                " | "
                a { href: "https://github.com/bmevprog/vbot/pull/2", "PR #2" }
            },
            p { {t!("vbot-description")} }
        }
    }
}

#[component]
fn Awards() -> Element {
    rsx! {
        CvEntry {
        heading: rsx! { strong { {t!("school-awards")} } },
        for (grade, year) in [(12, 2022), (11, 2021), (10, 2020)] {
                CvEntry { key: "{year}", shaded: false,
                    heading: rsx! { strong { {t!("physics-award", grade: grade)} } " | {year}" },
                    p { em {
                        if year == 2022 {
                            {t!("physics-european")}
                        } else {
                            {t!("physics-international", year: year)}
                        }
                    } }
                }
        }
        CvEntry { shaded: false,
            heading: rsx! { strong { {t!("math-award")} } },
            p { {t!("math-description")} }
        }
        CvEntry { shaded: false,
            heading: rsx! { strong { {t!("participation")} } },
            p { {t!("math-olympiad")} }
        }
        }
    }
}

#[component]
fn Skills() -> Element {
    let rows = [
        ("programming-languages", "programming-skills"),
        ("systems", "systems-skills"),
        ("databases", "database-skills"),
        ("theory", "theory-skills"),
    ];
    rsx! {
        DataTable {
            thead { tr { th { {t!("category")} } th { {t!("skills")} } } }
            tbody {
                for (category, description) in rows {
                    CvTableRow { key: "{category}", label: t!(category), td { {t!(description)} } }
                }
            }
        }
    }
}

#[component]
fn Languages() -> Element {
    rsx! {
        DataTable {
            thead { tr { th { {t!("language")} } th { {t!("level")} } th { {t!("evidence")} } } }
            tbody {
                CvTableRow { label: t!("albanian"), columns: 2, td { {t!("native")} } td { "–" } }
                CvTableRow { label: t!("english"), columns: 2, td { {t!("fluent")} } td { {t!("english-evidence")} } }
                CvTableRow { label: t!("german"), columns: 2, td { {t!("basic-a2")} } td { "Goethe-Zertifikat A2 [88/100]" } }
                CvTableRow { label: t!("hungarian"), columns: 2, td { {t!("basic")} } td { {t!("hungarian-evidence")} } }
            }
        }
    }
}
