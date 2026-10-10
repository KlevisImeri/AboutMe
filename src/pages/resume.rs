use crate::components::{Content, DataTable, SectionHeading};
use dioxus::prelude::*;

#[component]
pub fn ResumePage() -> Element {
    rsx! {
        Content {
            ContactHeader {}
            PersonalData {}
            Education {}
            Research {}
            Experience {}
            SideProjects {}
            Awards {}
            Skills {}
            Languages {}
            hr {}
        }
    }
}

#[component]
fn ContactHeader() -> Element {
    rsx! {
        header { class: "relative mx-auto text-center",
            div { class: "inline-block text-center",
                h1 { class: "contact-name mb-[0.5em] text-2xl font-bold", "Klevis Imeri" }
                div {
                    a { href: "https://aboutme.klevis.xyz/", "https://aboutme.klevis.xyz/" }
                }
                div { class: "mt-[0.5em] text-base",
                    span { class: "contact-item", "klevisimeri11@gmail.com" }
                    span { class: "separator mx-[0.7em] text-[#ccc]", "|" }
                    span { class: "contact-item", "+383 44 165 634" }
                    span { class: "separator mx-[0.7em] text-[#ccc]", "|" }
                    a {
                        href: "https://github.com/KlevisImeri/",
                        class: "contact-item contact-github",
                        "github.com/KlevisImeri"
                    }
                }
            }
        }
    }
}

#[component]
fn PersonalData() -> Element {
    rsx! {
        SectionHeading { title: "Personal Data" }
        DataTable {
            thead {
                tr {
                    th { "Date of birth" }
                    th { "Place of birth" }
                    th { "Nationality" }
                    th { "Marital status" }
                }
            }
            tbody {
                tr {
                    td { "08.05.2004" }
                    td { "Prishtina, Kosovo" }
                    td { "Kosovar & Albanian" }
                    td { "single" }
                }
            }
        }
    }
}

#[component]
fn Education() -> Element {
    rsx! {
        SectionHeading { title: "Education" }
        DataTable {
            thead {
                tr {
                    th { a { href: "https://ethz.ch/en.html", "ETH Zurich" } }
                    th { "Computer Science MSc" }
                    th { "start on 2026.09" }
                }
            }
        }
        DataTable {
            thead {
                tr {
                    th {
                        a { href: "https://www.bme.hu/en",
                            "Budapest University of Technology and Economics (BME)"
                        }
                    }
                    th { "Computer Engineering BSc 2022.09.01 - 2026.01.31" }
                    th { "Graduated excellent with highest honours" }
                    th { "GPA: 4.97/5.00" }
                    th { "Stipendium Hungaricum Scholarship" }
                }
            }
        }
        DataTable {
            thead {
                tr {
                    th { a { href: "https://mileniumi3.net/", "Mileniumi i Tretë Gymnasium" } }
                    th {
                        "Secondary High School Diploma, Natural Sciences"
                        br {}
                        "2019.09 – 2022.05"
                    }
                    th { "Student and Ambassador of the Generation" }
                    th { "GPA: 5.00/5.00" }
                }
            }
        }
    }
}

#[component]
fn Research() -> Element {
    rsx! {
        SectionHeading { title: "Research & Publications" }
        ul {
            li {
                strong { "1st Place – Scientific Student Association Conference (TDK)" }
                " | 2025"
                ul {
                    li { em { "BME, Faculty of Electrical Engineering and Informatics" } }
                    li { "Paper: “Improving Model Checking Portfolio Efficiency through Partial Result Transfer”" }
                    li { em { "Section: Software" } }
                    li {
                        a {
                            href: "https://tdk.bme.hu/conference/VIK/2025/sessions/sw/paper/Modellellenorzesi-portfoliok-hatekonysaganak",
                            "[View Paper]"
                        }
                    }
                }
            }
            li {
                strong { "BSc Thesis" }
                " | 2025"
                ul {
                    li { "Expanded the TDK research into a full BSc Thesis." }
                    li { a { href: "thesis.pdf", "[View Thesis]" } }
                }
            }
            li {
                strong { "Lab Report: Software Witness 2.0 Implementation" }
                " | 2025"
                ul {
                    li { "BME, Fault-Tolerant Systems Research Group (FTSRG)" }
                    li { "Integrated a new witness format using the product automaton of XCFAs into the Theta framework." }
                    li { a { href: "witness-theta-paper.pdf", "[View Report]" } }
                }
            }
            li {
                strong { "Publication: San Vitale Challenge" }
                " | 2024"
                ul {
                    li { em { "Automatic Reconstruction of Ancient Colored Glass Windows" } }
                    li { "Co-authored research paper with the Vprog Competitive Programming Group." }
                    li { "Published in " em { "Springer Nature" } "." }
                    li {
                        a { href: "https://link.springer.com/chapter/10.1007/978-3-031-91572-7_16",
                            "[View Chapter]"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Experience() -> Element {
    rsx! {
        SectionHeading { title: "Experience" }
        br {}
        p {
            strong { "Software Formal Verification Researcher" }
            " | " em { "December 2024 – Present" } " | Budapest BME"
        }
        ul {
            li {
                "Enhancing the efficiency and extending the capabilities of the "
                strong { a { href: "https://github.com/ftsrg/theta", "Theta" } }
                " validator developed at "
                strong { a { href: "https://ftsrg.mit.bme.hu/hu/", "FTSRG at BME" } }
                ", under the supervision of "
                strong { a { href: "https://adamzsofi.github.io/", "Zsófia Ádám" } }
                "."
            }
            li { "Collaborating closely with PhD researchers and contributors to navigate the framework’s complex codebase, ensuring alignment with formal methods and system architecture requirements." }
            li {
                strong { "Key contributions:" }
                ul {
                    li {
                        strong { "Model Checking Portfolio Optimization" }
                        " — enhancing portfolio efficiency by transferring partial results between algorithms such as CEGAR (Expl and Pred), BMC, KIND, and IMC, while introducing a novel stopping heuristic (TDK 1st Prize, BSc Thesis)."
                    }
                    li {
                        strong {
                            a {
                                href: "https://www.sosy-lab.org/research/pub/2024-SPIN.Software_Verification_Witnesses_2.0.pdf",
                                "Software Witness 2.0"
                            }
                            " Implementation"
                        }
                        " — integrating a new witness format using the product automaton of XCFAs."
                    }
                    li { "Optimizing IMC in Theta." }
                }
            }
            li {
                "All implementations were benchmarked on the "
                strong {
                    a { href: "http://benchcloud.mit.bme.hu:8088/benchcloud/webclient/master/info",
                        "Theta BenchCloud"
                    }
                }
                " infrastructure using the "
                strong { a { href: "https://gitlab.com/sosy-lab/benchmarking/sv-benchmarks", "SV-Benchmarks" } }
                " test suite, powered by "
                strong { a { href: "https://github.com/sosy-lab/benchexec", "BenchExec" } }
                " for precise, reproducible performance tracking."
            }
        }
        p {
            strong { a { href: "https://www.db.bme.hu/databases/", "DB Lab Instructor BME" } }
            " | " em { "September 2024 – January 2026" } " | Budapest BME"
        }
        ul {
            li { "Mentored students in DBMS fundamentals, covering ER modeling, schema design, normalization, relational algebra, and advanced SQL querying (including access control and integrity), while reviewing lab assessments for accuracy." }
        }
        p {
            strong { "Full-Stack Engineer" }
            " | " em { "January 2025 – September 2025" }
            " | Germany Stuttgart Online and In-person during Summer"
        }
        ul {
            li { "Developed and maintained a full-stack application using Vue.js and ASP.NET Core to manage worker logs, calculate wages, and generate GDPR-compliant audit reports for Königs-Villen Immobilien GmbH." }
        }
        p {
            strong { "Competitive Programmer" }
            " | " em { "January 2023 – Present" } " | Budapest"
        }
        ul {
            li {
                "Part of BME’s "
                a { href: "https://vprog.hu/", "Vprog" }
                " competitive programming group, achieving 2nd place in "
                a { href: "https://cs.bme.hu/icpc/", "ICPC Hungary" }
                " and advancing to "
                a { href: "https://cerc.acm.si/", "CERC 2023 regionals" }
                "."
            }
            li { "Deepened expertise in algorithms & data structures." }
            li {
                "Weekly participation in "
                a { href: "https://vprog.hu/calendar/", "Vprog Advance" }
                " (live sessions discussing algorithms with my peers at BME)"
            }
            li { "Co-developed software and published research, contributing to projects like Qubo and San-Vitale." }
            li {
                em { "ICPC Hungarian National Programming Contest:" }
                ul {
                    li { strong { "2025:" } " 5th Place" }
                    li { strong { "2024:" } " 6th Place" }
                    li {
                        strong { "2023:" }
                        " 2nd Place – Qualified for the Central European Regional Contest (CERC)"
                    }
                }
            }
        }
    }
}

#[component]
fn SideProjects() -> Element {
    rsx! {
        SectionHeading { title: "My Side Projects" }
        p {
            strong { a { href: "https://github.com/KlevisImeri/SimFN", "SimFN" } }
            " & "
            strong { a { href: "https://github.com/KlevisImeri/Integrator2", "Integrator2" } }
            " | Java,C/C++"
        }
        ul {
            li { "A graphing calculator that takes string input to plot functions." }
            li {
                "Implemented a:"
                ul {
                    li {
                        strong { "Lexer (Tokenizer)" }
                        ": Breaks down the string into keywords."
                    }
                    li {
                        strong { "Parser" }
                        ": Created an Abstract Syntax Tree (AST) using the "
                        a { href: "https://en.wikipedia.org/wiki/Shunting_yard_algorithm", "shunting yard algorithm" }
                        " to convert infix notation to postfix for easier tree construction."
                    }
                    li {
                        strong { "Evaluator" }
                        ": The function that evaluates the AST for various input values."
                    }
                }
            }
            li { "Everything, including the AST, was implemented from scratch. Swing was used for the GUI." }
            li {
                strong { "Integrator2" }
                " is an older version of this project implemented in C and C++, which also includes numeric integration."
            }
        }
        p {
            strong { a { href: "https://klevisimeri.github.io/BlochSphere/", "BlochSphere" } }
            " | Svelte, Bun, Vite.js, Three.js"
        }
        ul {
            li { "Interactive quantum Bloch sphere simulator in browser." }
            li { "Supports matrix-vector ops, real-time 3D visualization, and custom JS scripting." }
            li { "Used Threlte + Math.js for rendering and computation." }
            li {
                "You can find more information in the "
                a {
                    href: "https://github.com/KlevisImeri/BlochSphere/blob/main/BlochSphereSimulator.pdf",
                    "specification"
                }
                "."
            }
        }
    }
}

#[component]
fn Awards() -> Element {
    rsx! {
        SectionHeading { title: "Other Awards" }
        h4 { "[High School Awards]" }
        ul {
            li {
                strong { "1st Place – National Physics Competition (Grade 12)" }
                " | 2022"
                ul {
                    li { em { "Represented Kosovo in the International Physics Olympiad (IPhO) 2022 and the European Physics Olympiad (EuPhO) 2022." } }
                }
            }
            li {
                strong { "1st Place – National Physics Competition (Grade 11)" }
                " | 2021"
                ul {
                    li { em { "Represented Kosovo in the International Physics Olympiad (IPhO) 2021." } }
                }
            }
            li {
                strong { "1st Place – National Physics Competition (Grade 10)" }
                " | 2020"
                ul {
                    li { em { "Represented Kosovo in the International Physics Olympiad (IPhO) 2020." } }
                }
            }
            li {
                strong { "1st Place – National Competition of Knowledge (Mathematics)" }
                ul {
                    li {
                        "Achieved 1st place at both "
                        strong { "Municipal" }
                        " and "
                        strong { "School" }
                        " levels (Grade IX)."
                    }
                }
            }
            li {
                strong { "Certificate of Participation" }
                ul { li { "XI Mathematical Olympiad of Kosovo (Grade IX)." } }
            }
        }
    }
}

#[component]
fn Skills() -> Element {
    let skills = [
        (
            "Languages",
            "Rust, C, TypeScript, Lean, Rocq, React + Signals and their ecosystems",
        ),
        (
            "Systems & Tools",
            "Linux (Fedora), Git, Neovim, Docker, Kubernetes and their ecosystems",
        ),
        (
            "Databases",
            "PostgreSQL, SQLite, Oracle and their ecosystems",
        ),
        (
            "Theory",
            "Formal Verification, Information Security, Algorithm Optimization",
        ),
    ];

    rsx! {
        SectionHeading { title: "Skills" }
        DataTable {
            thead { tr { th { "Category" } th { "Skills" } } }
            tbody {
                for (category, description) in skills {
                    tr { key: "{category}", td { "{category}" } td { "{description}" } }
                }
            }
        }
    }
}

#[component]
fn Languages() -> Element {
    let languages = [
        ("Albanian", "Native", "-"),
        (
            "English",
            "Fluent (C2)",
            "BSc studies fully in English; TOEFL iBT [107] (2022)",
        ),
        ("German", "Basic (A2)", "Goethe-Zertifikat A2 [88/100]"),
        (
            "Hungarian",
            "Basic",
            "3.5 years living in Budapest, Hungary",
        ),
    ];

    rsx! {
        h4 { "[Language Skills]" }
        DataTable {
            thead { tr { th { "Language" } th { "Level" } th { "Certificate / Evidence" } } }
            tbody {
                for (language, level, evidence) in languages {
                    tr { key: "{language}",
                        td { "{language}" }
                        td { "{level}" }
                        td { "{evidence}" }
                    }
                }
            }
        }
    }
}
