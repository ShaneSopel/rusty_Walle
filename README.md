<<<<<<< HEAD
# rusty_Walle 🤖

Embedded Rust robotics firmware running on **ESP32 + FreeRTOS** (`esp-idf-hal` / `esp-idf-svc`) to control a 3D-printed **WALL-E** robot (based on chillibasket's WALL-E v3.0 mechanical design).

Built around a **hardware-agnostic, trait-based systems architecture** that decouples anatomical robot poses and expressions from underlying PWM/UART actuator controllers and DC motor drivers.

---

## 🏗️ Software & Systems Architecture

* **Runtime & Stack:** `std`-enabled Rust on **ESP-IDF / FreeRTOS** (`esp-idf-hal` `0.46+`, `esp-idf-svc`), deterministic task scheduling, and thread-safe shared bus access (`Arc<Mutex<...>>`).
* **Hardware Abstraction Layer (`src/actuators/`):**
  * **`ServoActuator` Trait:** Unified interface (`set_position(0.0..=1.0)`, `set_speed`, `set_accel`, `limp`) shared across heterogeneous servo backends.
  * **`MaestroBus` & `MaestroChannel` (`pololu_servo.rs`):** Thread-safe UART driver (`115,200` baud, Compact Serial Protocol with `0xAA` auto-baud initialization) controlling 6 upper-body servos with hardware trapezoidal speed/acceleration profiling.
  * **`LedcServo` (`ledc_servo.rs`):** Hardware-generic ESP32 LEDC PWM driver (`50 Hz`, 14-bit duty resolution) driving the 7th servo channel directly from the microcontroller.
  * **`Tb67h450Motor` (`tb67h450.rs` - In Progress):** `DcMotor` trait implementation for dual onboard Toshiba TB67H450FNG H-bridge tread drivers.
* **Appendage & Kinematic Layer (`src/appendage_controller/`):**
  * **`WallEBody` (`body.rs`):** High-level anatomical joint controller (`head_rotation`, `neck_top`, `neck_bottom`, `eye_left`, `eye_right`, `arm_left`, `arm_right`) storing `Box<dyn ServoActuator>` so higher-level behaviors remain agnostic to physical pin/channel assignments.
  * **`WallEMap` & `JointCal` (`mapping.rs`):** Centralized per-joint calibration (`min_us`, `max_us`, `inverted`, `home`) and channel-to-joint binding table.

---

## 🔧 Hardware Bill of Materials (BOM)

| Subsystem | Component | Interface / Notes |
| :--- | :--- | :--- |
| **Compute / MCU** | Emakefun Maker-ESP32 | Dual-core Xtensa LX6, FreeRTOS, onboard H-bridges |
| **Servo Controller** | Pololu Micro Maestro 6-Channel | UART (`GPIO 18` TX $\rightarrow$ Maestro `RX`, `115,200` baud), common `GND` |
| **Upper-Body Actuators** | 7x MG90S Micro Servos | Channels `0–5` on Pololu Maestro; Channel `7` on ESP32 `GPIO 25` (LEDC) |
| **Drive Motors** | 2x 12V Geared DC Tread Motors | Driven via Maker-ESP32 onboard Toshiba TB67H450FNG H-bridges |
| **Power Architecture** | Split Logic & Actuator Rails | Dedicated 5V UBEC feeding Maestro `VSRV`/`BAT` rail; isolated 3.3V MCU logic with shared ground |
| **Chassis** | 3D-Printed WALL-E v3.0 | Printed in PLA on Bambu Lab P2S |

---

## ✅ Systems Engineering & Integration Status

- [x] **End-to-End UART Servo Control:** Verified Compact Serial Protocol framing, `0xAA` auto-baud handshake, speed/acceleration profiling, and physical MG90S actuation across all 6 Pololu Maestro channels (`0..=5`).
- [x] **Direct ESP32 LEDC Servo Driver:** Implemented and bench-verified 50 Hz 14-bit `LedcServo` on `GPIO 25` conforming to `ServoActuator`.
- [x] **Split Power Rail Validation:** Verified external servo power injection (`VSRV`/`BAT`) with common ground isolation to prevent MCU brownouts.
- [ ] **Appendage Controller Integration:** Bind calibrated `WallEMap` channels into `WallEBody` and implement coordinated expressions (`look_at`, `tilt_eyes`, `wave`).
- [ ] **Tread Drive & Differential Kinematics:** Complete `tb67h450.rs` (`20 kHz` LEDC PWM) and differential tread mixer.
- [ ] **Full Chassis Assembly & Wiring Harness:** Final mechanical horn indexing at `0.5` neutral home and internal harness routing.

---

## 🚀 Future Work & Subsystem Roadmap

* **Eye Vision System (Binocular / Eye Cameras):**
  * Integrate compact camera modules inside WALL-E's eye housings for live video streaming, face/object tracking, and closed-loop head/eye pan-tilt tracking via `WallEBody`.
* **Audio & Sound Effects Subsystem (MP3 / I2S Audio):**
  * Add an MP3 playback module (e.g., DFPlayer Mini over UART or an I2S DAC/amplifier) triggered asynchronously from FreeRTOS tasks to play WALL-E vocalizations, startup chimes, and servo/motor sound effects synchronized with expressions.
* **Front Chassis Diagnostics & Charge-Meter Display (LCD Screen):**
  * Mount an SPI/I2C LCD panel on the front torso styled after WALL-E's iconic solar charge display, serving as a live telemetry and diagnostics screen (battery voltage, rail status, active FreeRTOS tasks, UART/motor health, and expression state) rendered in Rust (`embedded-graphics`).
=======
# rusty_Walle
# SW@GL0RD WALL-E — Rust Firmware (ESP32 + FreeRTOS)

Modular embedded Rust firmware (`esp-idf-hal` / `esp-idf-svc` on FreeRTOS) for a 3D-printed WALL-E robot powered by an **Emakefun Maker-ESP32**, a **Pololu Micro Maestro 6-channel servo controller**, an onboard **LEDC PWM servo**, and a **1.8" ST7735 SPI TFT diagnostics HUD**.

---

## Hardware Architecture & Pinout

### 1. Servo Actuators (7-DOF Upper Body)
* **Channels `0..=5` (Pololu Micro Maestro):** Driven over hardware `UART1` at **115,200 baud** using the Pololu Compact Serial Protocol (`init_auto_baud()`).
* **Channel `6` (`arm_right`):** Driven directly from the ESP32 `LEDC` peripheral (`Timer0`, `Channel0`) at **50 Hz** with **14-bit duty resolution**.

| Joint Name | Controller | Channel / Pin | Default Range ($\mu\text{s}$) |
| :--- | :--- | :--- | :--- |
| `head_rotation` | Pololu Maestro | Channel `0` | `500 – 2500` |
| `neck_top` | Pololu Maestro | Channel `1` | `500 – 2500` |
| `neck_bottom` | Pololu Maestro | Channel `2` | `500 – 2500` |
| `eye_left` | Pololu Maestro | Channel `3` | `500 – 2500` |
| `eye_right` | Pololu Maestro | Channel `4` | `500 – 2500` |
| `arm_left` | Pololu Maestro | Channel `5` | `500 – 2500` |
| `arm_right` | ESP32 `LEDC` | `GPIO 25` | `500 – 2500` |

### 2. Maker-ESP32 Wiring Map
> **Hardware Note:** On the Emakefun Maker-ESP32, ensure the onboard Motor/IO slide switches are set to **`IO`** so `GPIO 14`, `GPIO 15`, and `GPIO 17` operate as standard digital GPIOs.

| Peripheral | Signal | Maker-ESP32 Pin | Notes |
| :--- | :--- | :--- | :--- |
| **Pololu Maestro** | `TX` $\rightarrow$ Maestro `RX` | `GPIO 18` | `UART1` @ `115,200` baud |
| **Pololu Maestro** | `RX` *(unused)* | `GPIO 19` | Unidirectional TX control |
| **Right Arm Servo** | `PWM` Signal | `GPIO 25` | 50 Hz `LEDC` (`Timer0` / `Ch0`) |
| **ST7735 TFT** | `VCC` / `BLK (LED)` | `3.3V` | Logic power & full backlight |
| **ST7735 TFT** | `GND` | `GND` | Common ground across ESP32, Maestro & TFT |
| **ST7735 TFT** | `SCL` (`SCLK`) | `GPIO 14` | `SPI2` Clock (`26 MHz`) |
| **ST7735 TFT** | `SDA` (`MOSI`) | `GPIO 12` | `SPI2` Data (keep free of external pull-ups) |
| **ST7735 TFT** | `CS` | `GPIO 15` | Chip Select |
| **ST7735 TFT** | `A0` (`DC`) | `GPIO 26` | Data / Command select |
| **ST7735 TFT** | `RST` (`RES`) | `GPIO 17` | Hardware Reset |

---

## Repository Structure

The firmware separates low-level hardware drivers from robot-level kinematics and telemetry:

```text
src/
├── main.rs                      # Peripheral setup, SPI/UART init, & main pose loop
├── actuators/                   # Hardware-agnostic actuator traits & drivers
│   ├── mod.rs                   # ServoActuator trait (set_position, speed, accel, limp)
│   ├── pololu_servo.rs          # Thread-safe MaestroBus (Arc<Mutex>) & MaestroChannel
│   └── ledc_servo.rs            # ESP32 50 Hz LEDC PWM servo driver
├── appendage_controller/        # Anatomical robot kinematics & calibration
│   ├── mod.rs
│   ├── body.rs                  # WallEBody (7 joints + live TelemetryState updates)
│   └── mapping.rs               # WallEMap & JointCal central calibration table
├── display/                     # Hardware display abstraction & drivers
│   ├── mod.rs                   # DiagnosticsDisplay trait
│   └── st7735.rs                # 1.8" ST7735 SPI TFT renderer (160x128 Landscape)
└── diag/                        # Non-blocking background telemetry subsystem
    ├── mod.rs                   # FreeRTOS HUD task spawner (10 Hz refresh)
    └── telemetry.rs             # Shared TelemetryState struct
```

---

## Diagnostics HUD (`160 × 128` Landscape)
A dedicated FreeRTOS background thread (`diag::spawn_diagnostics_task`) renders live state onto the 1.8" ST7735 at **10 Hz** without blocking servo motion:
* **Top Banner:** `SW@GL0RD WALL-E` (Cyan)
* **Solar Charge Meter:** 10-segment color bar indicator (Red / Yellow / Green)
* **Live Telemetry:** Active pose name, Maestro UART bus status, normalized (`0.00–1.00`) joint positions for head yaw, upper/lower neck, left/right eyes, left/right arms, and charge percentage.

---

## Build & Flash

1. **Source the ESP-IDF / Rust Xtensa environment:**
   ```bash
   . ~/export-esp.sh
   ```
2. **Build and flash to the Maker-ESP32:**
   ```bash
   cargo run --release
   ```

---

## Roadmap
* [x] Pololu Micro Maestro 6-channel UART driver (`0..=5`)
* [x] 7th joint `LEDC` PWM driver (`GPIO 25`)
* [x] Unified `WallEBody` & `WallEMap` calibration layer
* [x]  1.8" ST7735 SPI diagnostics HUD (`160 × 128` Landscape)
* [ ] Dual DC tread motor driver (`TB67H450FNG` H-bridge PWM)
* [ ] Wireless remote control integration (Heltec V3 LoRa / ESP-NOW)
* [ ] Audio playback & eye camera modules
>>>>>>> 879bdad (added servo appenge updates and the LCD Diagnostics Screen)
