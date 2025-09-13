import{_ as e}from"./MarkdownRenderer.vue_vue_type_script_setup_true_lang-YxJnBujK.js";import{d as n,c as t,u as i,o as a}from"./index-CiHukJAr.js";const r=`<div style="position: relative; margin: 0 auto; text-align: center;">
  <div style="display: inline-block; text-align: center;">
    <div style="font-size: 1.5rem; font-weight: bold; margin-bottom: 0.5em;">Klevis Imeri</div>
    <!-- <div>20, Male</div> -->
    <div>
      <a href="https://github.com/KlevisImeri">https://github.com/KlevisImeri/</a>
    </div>
    <div>klevisimeri11@gmail.com | +36705384797</div>
  </div>

  <!-- <img src="../../public/Portrait.jpg"  -->
  <!--   alt="Portrait" -->
  <!--   style=" -->
  <!--     position: absolute; -->
  <!--     right: -0.01rem; -->
  <!--     top: 50%; -->
  <!--     transform: translateY(-34%); -->
  <!--     width: 8rem; -->
  <!--     height: 12.5rem; -->
  <!--     object-fit: cover; -->
  <!--     border-radius: 0.5rem; -->
  <!--     border: 0.125rem solid #e0e0e0; -->
  <!--   "> -->
</div>

---
#### Experience
<br>

[**Theta Model Checking Framework Contributor**](https://github.com/ftsrg/theta) | *December 2024 – Present* | Budapest BME
- Designing and implementing a validator for C program witnesses under the [SV-COMP 2.0](https://www.sosy-lab.org/research/pub/2024-SPIN.Software_Verification_Witnesses_2.0.pdf) format, leveraging product automata to verify reachability error properties and ensure formal soundness of verification results.
- Collaborating closely with PhD researchers and contributors to navigate the framework’s complex codebase, ensuring alignment with formal methods and system architecture requirements.
- Optimizing interpolation-based model checking (IMC) algorithms to accelerate bounded verification tasks, reducing computational overhead for complex program analyses.
- Running performance benchmarks on [BME Cloud](https://cloud.bme.hu/) virtualized infrastructure using the SV-COMP witness suite, leveraging [BenchExec](https://github.com/sosy-lab/benchexec) for precise resource tracking and reproducible testing.
<br>

[**DB Lab Instructor BME**](https://www.db.bme.hu/databases/) | *September 2024 – December 2024* | Budapest BME 
- Assisted students with SQL and database concepts, including:
     - Database Management Systems (DBMS) fundamentals,
     - Entity-Relationship (ER) Modeling and schema design,
     - Relational algebra and its translation to SQL queries,
     - CRUD operations (Create, Read, Update, Delete) and advanced SQL commands,
     - User authorization, privileges, and access control (e.g., GRANT/REVOKE permissions),
     - Normalization and database design best practices,
     - Data integrity constraints (primary/foreign keys, unique checks).
- Published a [FAQ website](https://klevisimeri.github.io/DB-Lab/) to address common questions.
- Reviewed lab entry tests for clarity and accuracy.
<br>

**Full-Stack Engineer – End-to-End Solutions** | *January 2025 - Present* | Stuttgart Online 
- Developed and maintain a full-stack application to manage worker logs, calculate work hours, and determine wages for 
[königs-villen immobilien gmbh](https://koenigs-villen-immobilien.de/). The product is an Internal-use platform.
- Key Responsibilities:
    - **Full-Stack Development:**
      - Build UI components using Vue.js and develop RESTful APIs with C# (.NET Core).  
      - Ensure smooth integration between front-end and back-end systems.
    - **Database Management & Auditing:**
      - Design and optimize SQLite database schemas.
      - Conduct regular database audits and maintain snapshots.
    - **CI/CD & Deployment:**  
      - Implement automated CI/CD pipelines for testing and deployment.  
      - Manage cloud-based deployments across workplace devices.
    - **Version Control & Security:**
      - Use GitHub for version control and collaborative development.  
      - Enforce robust security measures including authentication, authorization, and encryption.
    - **Reporting & Compliance:**
      - Generated audit reports with GDPR-mandated documentation.
      - Generate clear, professional audit documents and activity logs for compliance and payroll.
<br>

**Competitive Programmer** | *January 2023 – Present* | Budapest
- Part of BME’s [Vprog](https://vprog.hu/) competitive programming group, achieving 2nd place in [ICPC Hungary](https://cs.bme.hu/icpc/posts/2024-10-01-icpc-hungary/) and advancing to [CERC 2023 regionals](https://cerc.acm.si/).
- Deepened expertise in algorithms & data structures**, including:  
  - **Data Structures**: Vectors, linked lists, binary trees, segment trees, Fenwick trees, Link/cut tree, B*-tree, adjacency lists/matrices for graphs ...
  - **Graph Algorithms**: BFS, DFS, A*, Dijkstra’s, Floyd-Warshall, graph isomorphism, network flow (Ford-Fulkerson) ... 
  - **Dynamic Programming**: Knapsack, LCS, bitmask DP ... 
  - **Number Theory & Math**: Modular arithmetic, sieve algorithms, FFT, combinatorics ...
  - **Other**: Mo’s algorithm, hashing, Divide-and-conquer ... 
- Frequent participation in the [Codeforces](https://codeforces.com/) competitions.
- Weekly participation in [Vprog Advance](https://vprog.hu/calendar/) (live sessions discussing algorithms with my peers at BME)
- We also develop software and publish research. I've contributed to projects like [Qubo](https://github.com/bmevprog/qubo) and [San-Vitale](https://github.com/bmevprog/san-vitale).

---
#### MyProjects
[**SimFN**](https://github.com/KlevisImeri/SimFN) & [**Integrator2**](https://github.com/KlevisImeri/Integrator2) | Java,C/C++ 
- A graphing calculator that takes string input to plot functions.
- I had to build a:
	- **Lexer (Tokenizer)**: Breaks down the string into keywords.
	- **Parser**: Created an Abstract Syntax Tree (AST) using the [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) to convert infix notation to postfix for easier tree construction.
	- **Evaluator**: The function that evaluates the AST for various input values.
- Everything, including the AST, was implemented from scratch. Swing was used for the GUI.
- This experience taught me the fundamentals of language creation and semantics in Java using an OOP paradigm.
- **Integrator2** is an older version of this project implemented in C and C++, which also includes numeric integration.

[**Pipes In The Desert**](https://github.com/KlevisImeri/Pipes-In-The-Desert) | Java
- A group software project with 4 members.
- Learned to collaborate effectively and manage complex version control.
- Project phases included requirements gathering, design, implementation, and testing.
- The project followed a design-first approach, involving:
	- **Model Design**: Created class diagrams to structure the model.
	- **Behavior Design**: Developed sequence diagrams to illustrate interactions.
	- **User Interaction Design**: Used use case diagrams to capture user interactions.
	- **Component Design**: Employed component diagrams to organize system dependencies.
- Developed custom testing methods and a specific testing project language.
- Swing was used for the UI. 

[**BlochSphere**](https://github.com/KlevisImeri/BlochSphere/blob/main/BlochSphereSimulator.pdf) | Svelte, Bun, Vite.js, Three.js
- Interactive quantum Bloch sphere simulator in browser. 
- Supports matrix-vector ops, real-time 3D visualization, and custom JS scripting. 
- Used Threlte + Math.js for rendering and computation.
- This is work under development. You can find more information in the [specification](https://github.com/KlevisImeri/BlochSphere/blob/main/BlochSphereSimulator.pdf).

[**CleanEdit**](https://github.com/KlevisImeri/CleanEdit) | TypeScript, Vue, Tailwind, C#, FFmpeg, Python 
- A fast, web-based video editor.
- **Frontend**: 
    - Built with Vue.js, leveraging reactivity (ref(), computed()), lifecycle hooks (onMount()), and DOM manipulation for dynamic UI interactions.
- **Backend**: 
    - Designed a RESTful API using C# ASP.NET Core with Entity Framework for database management and ORM capabilities.
    - Integrated FFmpeg and Python’s auto-edit package for video processing and auto-cutting.
    - Implemented a secure authentication system using JWT tokens and user sessions.
- **Styling**: Used Tailwind for efficient CSS styling.

> *Note:* **Many** other projects are available on my GitHub.

---
#### Education
| [Budapest University of Technology and Economics (BME)](https://www.bme.hu/en) | Computer Engineering | 6th Sem | GPA: 4.93/5.00 |
|:--------------------------------------------------------------------------|:---------------------|:--------|:----------------|

---
#### Skills
<div style="text-align: center;"><i>General</i></div>
<div style="text-align: center;"><div>DevOps | Full-Stack | Cloud Computing | Testing: Verification & Validation</div> <br>

<div style="text-align: center;"><i>Spoken languages</i></div>
<div style="text-align: center;">English – Fluent/Native-level |Albanian - Fluent | German - Basic</div> <br>

<div style="text-align: center;"><i>Programming languages & domain specific languages:</i></div>
<div style="text-align: center;">C/C++ | JavaScript/TypeScript | SQL/T-SQL | Java/Kotlin | C# | Python | Prolog | Lua</div> <br>

<div style="text-align: center;"><i>Databases:</i></div>
<div style="text-align: center;">Oracle | MS SQL | SQLite</div> <br>

<div style="text-align: center;"><i>OS development</i></div>
<div style="text-align: center;">Linux - Fedora | Windows10/11 | Android</div> <br>

<div style="text-align: center;"><i>Libraries, Frameworks or Platforms</i></div>
<div style="text-align: center;">Text: markdown, latex | C/C++: Raylib, OpenGL | Python: NumPy, Matplotlib, OpenCV, Pandas, TensorFlow, Scikit-learn | JS/TS: NodeJS, Bun, Vue, Electron | C#: Entity Framework, ASP.NET Core | Java/Kotlin:  Spring, Swing, Log4j, Gradle </div> <br>

<div style="text-align: center;"><i>Programing paradigms</i></div>
<div style="text-align: center;">OOP | PROC</div>

`,m=n({__name:"HomeView",setup(o){return(s,l)=>(a(),t(e,{content:i(r)},null,8,["content"]))}});export{m as default};
