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
                PersonalData {}
                Education {}
                Research {}
                Experience {}
                Projects {}
                Awards {}
                Skills {}
                Languages {}
                hr {}
            }
        }
    }
}

#[component]
fn Heading(id: &'static str) -> Element {
    let title = t!(id);
    rsx! { hr {} h4 { "[{title}]" } }
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
    rsx! {
        Heading { id: "personal-data" }
        div { class: "personal-data",
        DataTable {
            thead { tr {
                th { {t!("date-of-birth")} } th { {t!("place-of-birth")} }
                th { {t!("nationality")} } th { {t!("marital-status")} }
            } }
            tbody { tr {
                td { "08.05.2004" } td { {t!("birthplace")} }
                td { {t!("citizenship")} } td { {t!("single")} }
            } }
        }
        }
    }
}

#[component]
fn Education() -> Element {
    rsx! {
        Heading { id: "education" }
        div { class: "education-entries",
            div { class: "education-entry",
                p { class: "experience-heading", strong { {t!("eth-name")} } " | " {t!("eth-period")} " | "
                    a { href: "https://ethz.ch/en.html", {t!("view-website")} }
                }
                p { {t!("eth-degree")} }
            }
            div { class: "education-entry",
                p { class: "experience-heading", strong { {t!("bme-name")} } " | " {t!("bme-period")} " | "
                    a { href: "https://www.bme.hu/en", {t!("view-website")} }
                }
                p { {t!("bme-degree")} }
                p { class: "education-meta",
                    {t!("bme-honours")} " · " {t!("gpa", value: "4.97/5.00")} " · " {t!("past-scholarship")}
                }
            }
            div { class: "education-entry",
                p { class: "experience-heading", strong { {t!("school-name")} } " | " {t!("school-period")} " | "
                    a { href: "https://mileniumi3.net/", {t!("view-website")} }
                }
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
        Heading { id: "research" }
        p { class: "experience-heading",
            strong { {t!("tdk-title")} } " | 2025 | "
            a { href: "https://tdk.bme.hu/conference/VIK/2025/sessions/sw/paper/Modellellenorzesi-portfoliok-hatekonysaganak", {t!("view-paper")} }
        }
        p {
            em { {t!("tdk-faculty")} } br {}
            {t!("tdk-paper")} br {}
            em { {t!("tdk-section")} }
        }
        p { class: "experience-heading",
            strong { {t!("thesis-title")} } " | 2025 | "
            a { href: "https://aboutme.klevis.xyz/thesis.pdf", {t!("view-thesis")} }
        }
        p {
            {t!("thesis-description")}
        }
        p { class: "experience-heading",
            strong { {t!("witness-title")} } " | 2025 | "
            a { href: "https://aboutme.klevis.xyz/witness-theta-paper.pdf", {t!("view-report")} }
        }
        p {
            {t!("research-group")} br {}
            {t!("witness-description")}
        }
        p { class: "experience-heading",
            strong { {t!("publication-title")} } " | 2025 | "
            a { href: "https://link.springer.com/chapter/10.1007/978-3-031-91572-7_16", {t!("view-chapter")} }
        }
        p {
            em { "Automatic Reconstruction of Ancient Colored Glass Windows" } br {}
            {t!("publication-description")} br {}
            {t!("publication-publisher", publisher: "Springer Nature")}
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
        Heading { id: "experience" }
        p { class: "experience-heading",
            strong { {t!("researcher-title")} } " | " {t!("researcher-period")} " | BME, Budapest"
        }
        p {
            LinkedText { text: t!("researcher-description", software: "§theta§", group: "§ftsrg§", supervisor: "§supervisor§", witness: "§witness§", infrastructure: "§benchcloud§", suite: "§benchmarks§", tool: "§benchexec§") }
        }
        p { class: "experience-heading",
            strong { {t!("instructor-title")} }
            " | " {t!("instructor-period")} " | Budapest | "
            a { href: "https://www.db.bme.hu/databases/", {t!("view-website")} }
        }
        p { {t!("instructor-description")} }
        p { class: "experience-heading",
            strong { {t!("engineer-title")} } " | " {t!("engineer-period")} " | " {t!("engineer-location")}
        }
        p { {t!("engineer-description")} }
        p { class: "experience-heading",
            strong { {t!("competitive-title")} } " | " {t!("competitive-period")} " | Budapest"
        }
        p {
            LinkedText { text: t!("competitive-description", group: "§vprog§", practice: "Vprog Advance", contest: "§icpc§", regional: "§cerc§") }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        Heading { id: "projects" }
        p { class: "experience-heading",
            strong { "SimFN " {t!("and")} " Integrator2" }
            " | Java, C/C++ | "
            a { href: "https://github.com/KlevisImeri/SimFN", {t!("view-project", name: "SimFN")} }
            " | "
            a { href: "https://github.com/KlevisImeri/Integrator2", {t!("view-project", name: "Integrator2")} }
        }
        p { {t!("simfn-description")} br {} {t!("integrator-description")} }
        p { class: "experience-heading",
            strong { "BlochSphere" }
            " | Svelte, Bun, Vite.js, Three.js | "
            a { href: "https://klevisimeri.github.io/BlochSphere/", {t!("view-project", name: "BlochSphere")} }
        }
        p {
            {t!("bloch-description")} br {}
            {t!("bloch-features")} br {}
            {t!("bloch-tools")}
        }
    }
}

#[component]
fn Awards() -> Element {
    rsx! {
        Heading { id: "awards" }
        p { class: "experience-heading", strong { {t!("school-awards")} } }
        for (grade, year) in [(12, 2022), (11, 2021), (10, 2020)] {
                p { key: "{year}",
                    strong { {t!("physics-award", grade: grade)} } " | {year}"
                    br {}
                    em {
                        if year == 2022 {
                            {t!("physics-european")}
                        } else {
                            {t!("physics-international", year: year)}
                        }
                    }
                }
        }
        p { strong { {t!("math-award")} } br {} {t!("math-description")} }
        p { strong { {t!("participation")} } br {} {t!("math-olympiad")} }
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
        Heading { id: "skills" }
        DataTable {
            thead { tr { th { {t!("category")} } th { {t!("skills")} } } }
            tbody {
                for (category, description) in rows {
                    tr { key: "{category}", td { {t!(category)} } td { {t!(description)} } }
                }
            }
        }
    }
}

#[component]
fn Languages() -> Element {
    let title = t!("language-skills");
    rsx! {
        h4 { "[{title}]" }
        DataTable {
            thead { tr { th { {t!("language")} } th { {t!("level")} } th { {t!("evidence")} } } }
            tbody {
                tr { td { {t!("albanian")} } td { {t!("native")} } td { "–" } }
                tr { td { {t!("english")} } td { {t!("fluent")} } td { {t!("english-evidence")} } }
                tr { td { {t!("german")} } td { {t!("basic-a2")} } td { "Goethe-Zertifikat A2 [88/100]" } }
                tr { td { {t!("hungarian")} } td { {t!("basic")} } td { {t!("hungarian-evidence")} } }
            }
        }
    }
}
