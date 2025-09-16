
# ChessGame ♟️

**ChessGame** is a pedagogical project designed to help beginner developers—and Rust enthusiasts of all levels—explore Rust, WebAssembly, and CI/CD deployment. It’s a playable chess game running in your browser, with plenty of room to grow!

Whether you’re a student just learning Rust or a recruiter checking out a solid demo of your skills, this project is meant to teach, explore, and have a bit of fun along the way.

---

## Demo

Try it live on GitHub Pages: \[PLACEHOLDER\_LINK]

![ChessGame Demo](PLACEHOLDER_GIF)

---

## Goals 🎯

* Introduce Rust programming through a real, interactive project.
* Explore WebAssembly for web deployment.
* Learn CI/CD workflows with GitHub Actions.
* Build a foundation for more advanced projects: AI, multiplayer, and full-stack chess platforms.

---

## Features

* Play a basic chess game with another player on the same browser.
* Modular Rust code for board, GUI, move validation, threats, and PGN handling.
* CI/CD deployment using GitHub Actions and GitHub Pages.

> ⚠️ Note: Multiplayer, AI opponents, and move evaluation are on the roadmap!

---

## Installation

Since this project runs in the browser, installation isn’t necessary.
However, if you want to set up a local development environment, check the [Installation Guide](PLACEHOLDER_INSTALLATION_README).

---

## How to Play

Simply open the demo link in your browser and start playing. All moves are validated, and the game updates in real time.

---

## Project Structure

```text
src
├── board
│   └── ...
├── gui
│   └── ...
├── lib.rs
├── pgn
│   └── ...
├── threat
│   └── ...
└── validate_move
    └── ...
```

* **board** – Board logic and legal move calculation
* **gui** – All user interface components, panels, widgets
* **pgn** – Export and encode game history
* **threat** – Compute threatened cells
* **validate\_move** – Verify legal moves

---

## CI/CD Deployment

The project uses GitHub Actions to:

1. Build the Rust project into WebAssembly.
2. Automatically deploy the result to GitHub Pages.

This way, every commit keeps the live demo up-to-date.

---

## Contribution ✨

If you want to explore Rust and help, feel free to submit PRs! Some ideas for contributions:

* Improve GUI or add new widgets
* Implement move export/import (PGN)
* Add multiplayer functionality
* Optimize board or move evaluation logic
* Enhance CI/CD workflow and DevOps practices

> Don’t worry if you’re new—any contribution is a chance to learn Rust and web deployment!

---

## Roadmap 🛠️

1. Finish PGN export/import, add error handling and security.
2. Explore DevOps workflows and build a basic stack for development.
3. Add multiplayer support with a web server (C++ backend).
4. Implement basic move evaluation, multithreaded for server-side.
5. Add AI opponent on server side with multithreading.
6. Build a full-stack backend: user registration, matchmaking, Elo system, rankings.

---

## Learning Outcomes 📚

By exploring this project, you’ll gain:

* Rust fundamentals and modular project structuring
* WebAssembly deployment for browser-based apps
* CI/CD workflow with GitHub Actions
* GUI logic for games and interactive applications
* Basics of DevOps, networking, and low-level programming

