<h1 align="center"><a href="https://aboutme.klevis.xyz/">aboutme.klevis.xyz</a></h1>

## Rules

- Keep this README clean; it only contains rules.
- To run the dev server, build, or checks, use `cargo xtask` and nothing else:
  - `cargo xtask dev`
  - `cargo xtask build`
  - `cargo xtask check`
  - `cargo xtask serve`
  - `cargo xtask build serve`
  - `cargo xtask pdf [output.pdf]` exports the English CV (default: `CV_English.pdf`); run `cargo xtask serve` in another terminal first. Quote paths containing spaces.
  - `cargo xtask pdf-sq [output.pdf]` exports the Albanian CV at `/sq/` (default: `CV_Albanian.pdf`), with the same server requirement.
