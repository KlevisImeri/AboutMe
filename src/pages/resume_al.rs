use crate::components::{Content, DataTable, SectionHeading};
use dioxus::prelude::*;

#[component]
pub fn ResumeAlPage() -> Element {
    rsx! {
        Content {
            header { class: "relative mx-auto text-center", lang: "sq",
                h1 { class: "contact-name mb-[0.5em] text-2xl font-bold", "Klevis Imeri" }
                a { href: "https://aboutme.klevis.xyz/", "https://aboutme.klevis.xyz/" }
                p { "klevisimeri11@gmail.com | +383 44 165 634 | "
                    a { href: "https://github.com/KlevisImeri/", "github.com/KlevisImeri" }
                }
            }
            div { lang: "sq",
                SectionHeading { title: "Të dhënat personale" }
                DataTable {
                    thead { tr { th { "Data e lindjes" } th { "Vendi i lindjes" } th { "Shtetësia" } th { "Gjendja martesore" } } }
                    tbody { tr { td { "08.05.2004" } td { "Prishtinë, Kosovë" } td { "Kosovare dhe shqiptare" } td { "Beqar" } } }
                }
                SectionHeading { title: "Arsimi" }
                DataTable {
                    tbody {
                        tr {
                            th { a { href: "https://ethz.ch/en.html", "ETH Zürich" } }
                            td { "Master në Shkenca Kompjuterike (MSc)" }
                            td { "14 shtator 2026 – në vazhdim" }
                        }
                        tr {
                            th { a { href: "https://www.bme.hu/en", "Universiteti i Teknologjisë dhe Ekonomisë i Budapestit (BME)" } }
                            td { "Baçelor në Inxhinieri të Shkencave Kompjuterike (BSc) | 1 shtator 2022 – 6 janar 2026" }
                            td { "Diplomuar me nderimet më të larta" }
                            td { "Mesatarja: 4.97/5.00" }
                            td { "Bursa Stipendium Hungaricum" }
                        }
                        tr {
                            th { a { href: "https://mileniumi3.net/", "Gjimnazi Mileniumi i Tretë" } }
                            td { "Diplomë e shkollës së mesme, shkenca natyrore | shtator 2019 – maj 2022" }
                            td { "Nxënës dhe ambasador i gjeneratës" }
                            td { "Mesatarja: 5.00/5.00" }
                        }
                    }
                }
                SectionHeading { title: "Kërkimi shkencor dhe publikimet" }
                ul {
                    li {
                        strong { "Vendi i parë – Konferenca Shkencore Studentore (TDK) | 2025" }
                        ul {
                            li { "BME, Fakulteti i Inxhinierisë Elektrike dhe Informatikës; seksioni: Softuer" }
                            li { "Punimi: “Improving Model Checking Portfolio Efficiency through Partial Result Transfer”" }
                            li { a { href: "https://tdk.bme.hu/conference/VIK/2025/sessions/sw/paper/Modellellenorzesi-portfoliok-hatekonysaganak", "[Shiko punimin]" } }
                        }
                    }
                    li {
                        strong { "Tema e diplomës baçelor | 2025" }
                        ul {
                            li { "Zgjerova kërkimin e TDK-së në një temë të plotë diplome." }
                            li { a { href: "/thesis.pdf", "[Shiko temën]" } }
                        }
                    }
                    li {
                        strong { "Raport laboratorik: Implementimi i Software Witness 2.0 | 2025" }
                        ul {
                            li { "BME, Grupi Kërkimor për Sisteme Tolerante ndaj Defekteve (FTSRG)" }
                            li { "Integrova një format të ri të dëshmive të verifikimit në platformën Theta, duke përdorur automatin produkt të XCFA-ve." }
                            li { a { href: "/witness-theta-paper.pdf", "[Shiko raportin]" } }
                        }
                    }
                    li {
                        strong { "Publikim: San Vitale Challenge | 2024" }
                        ul {
                            li { em { "Automatic Reconstruction of Ancient Colored Glass Windows" } }
                            li { "Bashkautor i një punimi shkencor me grupin e programimit garues Vprog, botuar nga Springer Nature." }
                            li { a { href: "https://link.springer.com/chapter/10.1007/978-3-031-91572-7_16", "[Shiko kapitullin]" } }
                        }
                    }
                }
                SectionHeading { title: "Përvoja" }
                p { strong { "Studiues në verifikimin formal të softuerit" } " | dhjetor 2024 – në vazhdim | BME, Budapest" }
                ul {
                    li { "Përmirësoj efikasitetin dhe zgjeroj funksionalitetet e verifikuesit " a { href: "https://github.com/ftsrg/theta", "Theta" } ", të zhvilluar në " a { href: "https://ftsrg.mit.bme.hu/hu/", "FTSRG, BME" } ", nën mbikëqyrjen e " a { href: "https://adamzsofi.github.io/", "Zsófia Ádám" } "." }
                    li { "Bashkëpunoj me studiues të doktoraturës dhe kontribuues të tjerë në kodin e platformës, duke respektuar kërkesat e metodave formale dhe arkitekturës së sistemit." }
                    li { strong { "Kontributet kryesore:" }
                        ul {
                            li { "Optimizimi i portofolave të kontrollit të modeleve: transferimi i rezultateve të pjesshme ndërmjet algoritmeve CEGAR (Expl dhe Pred), BMC, KIND dhe IMC, si dhe zhvillimi i një heuristike të re ndalimi (vendi i parë në TDK; tema e diplomës)." }
                            li { a { href: "https://www.sosy-lab.org/research/pub/2024-SPIN.Software_Verification_Witnesses_2.0.pdf", "Implementimi i Software Witness 2.0" } ": integrimi i një formati të ri përmes automatit produkt të XCFA-ve." }
                            li { "Optimizimi i IMC-së në Theta." }
                        }
                    }
                    li { "Implementimet u testuan në infrastrukturën " a { href: "http://benchcloud.mit.bme.hu:8088/benchcloud/webclient/master/info", "Theta BenchCloud" } ", duke përdorur " a { href: "https://gitlab.com/sosy-lab/benchmarking/sv-benchmarks", "SV-Benchmarks" } " dhe " a { href: "https://github.com/sosy-lab/benchexec", "BenchExec" } " për matje të sakta dhe të riprodhueshme të performancës." }
                }
                p { strong { a { href: "https://www.db.bme.hu/databases/", "Instruktor i laboratorëve të bazave të të dhënave, BME" } } " | shtator 2024 – janar 2026 | Budapest" }
                ul { li { "Udhëzova studentët në bazat e sistemeve të menaxhimit të bazave të të dhënave: modelimi ER, projektimi i skemave, normalizimi, algjebra relacionale dhe SQL i avancuar, përfshirë kontrollin e qasjes dhe integritetin; kontrollova gjithashtu saktësinë e detyrave laboratorike." } }
                p { strong { "Inxhinier Full-Stack" } " | janar 2025 – shtator 2025 | Stuttgart, Gjermani — në distancë dhe me prani fizike gjatë verës" }
                ul { li { "Zhvillova dhe mirëmbajta një aplikacion me Vue.js dhe ASP.NET Core për regjistrimin e punës së punëtorëve, llogaritjen e pagave dhe gjenerimin e raporteve të auditimit në përputhje me GDPR-në për Königs-Villen Immobilien GmbH." } }
                p { strong { "Programues garues" } " | janar 2023 – në vazhdim | Budapest" }
                ul {
                    li { "Anëtar i grupit të programimit garues " a { href: "https://vprog.hu/", "Vprog" } " në BME; vendi i dytë në ICPC Hungari dhe kualifikim në garën rajonale CERC 2023." }
                    li { "Thellova njohuritë në algoritme dhe struktura të të dhënave." }
                    li { "Pjesëmarrje javore në " a { href: "https://vprog.hu/calendar/", "Vprog Advance" } ", me diskutime të drejtpërdrejta mbi algoritmet me kolegët në BME." }
                    li { "Bashkëzhvillova softuer dhe publikova kërkime, duke kontribuar në projekte si Qubo dhe San-Vitale." }
                    li { "Gara kombëtare hungareze e programimit ICPC:"
                        ul { li { "2025: vendi i pestë" } li { "2024: vendi i gjashtë" } li { "2023: vendi i dytë — kualifikim në CERC" } }
                    }
                }
                SectionHeading { title: "Projektet personale" }
                p { strong { a { href: "https://github.com/KlevisImeri/SimFN", "SimFN" } " dhe " a { href: "https://github.com/KlevisImeri/Integrator2", "Integrator2" } } " | Java, C/C++" }
                ul {
                    li { "Kalkulator grafik që lexon funksione në formë teksti dhe vizaton grafikët e tyre." }
                    li { "Implementova një analizues leksikor për ndarjen e tekstit në njësi, një analizues sintaksor që ndërton pemën sintaksore abstrakte (AST) me algoritmin shunting yard dhe një vlerësues që llogarit funksionin për vlera të ndryshme hyrëse." }
                    li { "Të gjithë komponentët, përfshirë AST-në, u implementuan nga e para. Ndërfaqja grafike u ndërtua me Swing." }
                    li { "Integrator2 është një version më i hershëm në C dhe C++, që përfshin edhe integrimin numerik." }
                }
                p { strong { a { href: "https://klevisimeri.github.io/BlochSphere/", "BlochSphere" } } " | Svelte, Bun, Vite.js, Three.js" }
                ul {
                    li { "Simulator kuantik ndërveprues i sferës Bloch në shfletues." }
                    li { "Mbështet veprime me matrica dhe vektorë, vizualizim 3D në kohë reale dhe skripte të personalizuara JavaScript." }
                    li { "Përdora Threlte dhe Math.js për vizualizim dhe llogaritje." }
                    li { a { href: "https://github.com/KlevisImeri/BlochSphere/blob/main/BlochSphereSimulator.pdf", "[Shiko specifikimin]" } }
                }
                SectionHeading { title: "Çmime të tjera" }
                h4 { "[Çmimet gjatë shkollimit parauniversitar]" }
                ul {
                    li { strong { "Vendi i parë – Gara kombëtare e fizikës, klasa XII | 2022" } ul { li { "Përfaqësova Kosovën në Olimpiadën Ndërkombëtare të Fizikës (IPhO) dhe Olimpiadën Evropiane të Fizikës (EuPhO) më 2022." } } }
                    li { strong { "Vendi i parë – Gara kombëtare e fizikës, klasa XI | 2021" } ul { li { "Përfaqësova Kosovën në IPhO 2021." } } }
                    li { strong { "Vendi i parë – Gara kombëtare e fizikës, klasa X | 2020" } ul { li { "Përfaqësova Kosovën në IPhO 2020." } } }
                    li { strong { "Vendi i parë – Gara e diturisë në matematikë" } ul { li { "Vendi i parë në nivel komunal dhe shkollor, klasa IX." } } }
                    li { "Certifikatë pjesëmarrjeje në Olimpiadën XI Matematike të Kosovës, klasa IX." }
                }
                SectionHeading { title: "Aftësitë" }
                DataTable {
                    thead { tr { th { "Kategoria" } th { "Aftësitë" } } }
                    tbody {
                        tr { td { "Gjuhët" } td { "Rust, C, TypeScript, Lean, Rocq, React + Signals dhe ekosistemet e tyre" } }
                        tr { td { "Sistemet dhe mjetet" } td { "Linux (Fedora), Git, Neovim, Docker, Kubernetes dhe ekosistemet e tyre" } }
                        tr { td { "Bazat e të dhënave" } td { "PostgreSQL, SQLite, Oracle dhe ekosistemet e tyre" } }
                        tr { td { "Teoria" } td { "Verifikimi formal, siguria e informacionit, optimizimi i algoritmeve" } }
                    }
                }
                h4 { "[Njohuritë gjuhësore]" }
                DataTable {
                    thead { tr { th { "Gjuha" } th { "Niveli" } th { "Certifikimi / Përvoja" } } }
                    tbody {
                        tr { td { "Shqip" } td { "Gjuhë amtare" } td { "–" } }
                        tr { td { "Anglisht" } td { "Rrjedhshëm (C2)" } td { "Studimet baçelor të përfunduara në anglisht; studimet master në anglisht" } }
                        tr { td { "Gjermanisht" } td { "Bazë (A2)" } td { "Goethe-Zertifikat A2 [88/100]" } }
                        tr { td { "Hungarisht" } td { "Bazë" } td { "3.5 vjet jetesë në Budapest, Hungari" } }
                    }
                }
                hr {}
            }
        }
    }
}
