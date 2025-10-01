# 3D Sternensystem – Rust, Glium & egui

Dieses Projekt ist ein **Proof of Concept** zur Darstellung eines **3D-Sonnensystems** in Rust.  
Es kombiniert **Glium** (für OpenGL-Rendering) mit **egui** (für interaktive Benutzeroberflächen), um eine flexible und erweiterbare Umgebung für die Simulation von Planetenbewegungen und Weltraumobjekten zu schaffen.  

## Features (geplant / teilweise umgesetzt)

- ☀️ **Zentrales Sternobjekt** (Sonne)  
- 🪐 **Orbitale Bewegung von Planeten**  
- 🌍 Unterschiedliche **Größen & Abstände** der Planeten  
- 🎥 **Kamera-Steuerung** (Rotation, Zoom, Bewegung im Raum)  
- 🖼️ **Rendering mit Glium** (OpenGL über Rust)  
- 🛠️ **Interaktive UI mit egui** (Parameter verändern, Debug-Infos, Simulation steuern)  
- 🗂️ **Asset-Ordner** für Texturen & Modelle  
- ✨ Erweiterbar für Monde, Asteroiden, Lichteffekte und Shader-Spielereien  

*(Liste kann nach und nach erweitert werden)*  

## Nutzung

### Voraussetzungen
- Rust-Toolchain (mindestens 2021 Edition)  
- Cargo als Build-System  
- Abhängigkeiten (werden in `Cargo.toml` gepflegt):  
  - `glium`  
  - `egui` + Integration (`egui_glium`)  

### Build & Run
```bash
cargo run
```

Nach dem Start öffnet sich ein Fenster mit:  
- einer 3D-Szene des Sternensystems  
- einer egui-Oberfläche zur Steuerung der Simulation  

## Projektstatus
Dies ist ein **Work in Progress**.  
Der Fokus liegt auf dem **Lernen und Ausprobieren von 3D-Rendering & UI in Rust**.  
Features werden Schritt für Schritt ergänzt.  

## Zukünftige Ideen
- 🌌 **Beleuchtung & Shader** (z. B. für Sonnenlicht, atmosphärische Effekte)  
- 🛰️ **Monde & Ringe**  
- 🚀 **Steuerbare Kamera-Flugpfade**  
- ⏱️ **Zeitskalierung** (Simulation schneller/langsamer ablaufen lassen)  
- 📡 **Physik-Simulation** (Gravitationskräfte, realistischere Bewegungen)  

## Lizenz
Ohne Gewähr – Projekt dient nur **Tests, Lernen und Experimentieren**.  
