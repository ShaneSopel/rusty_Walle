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

## 🚀 Future Work & Subsystem Roadmap

* **Eye Vision System (Binocular / Eye Cameras):**
  * Integrate compact camera modules inside WALL-E's eye housings for live video streaming, face/object tracking, and closed-loop head/eye pan-tilt tracking via `WallEBody`.
* **Audio & Sound Effects Subsystem (MP3 / I2S Audio):**
  * Add an MP3 playback module (e.g., DFPlayer Mini over UART or an I2S DAC/amplifier) triggered asynchronously from FreeRTOS tasks to play WALL-E vocalizations, startup chimes, and servo/motor sound effects synchronized with expressions.
* **Front Chassis Diagnostics & Charge-Meter Display (LCD Screen):**
  * Mount an SPI/I2C LCD panel on the front torso styled after WALL-E's iconic solar charge display, serving as a live telemetry and diagnostics screen (battery voltage, rail status, active FreeRTOS tasks, UART/motor health, and expression state) rendered in Rust (`embedded-graphics`).

## ✅ Systems Engineering & Integration Status

- [x] **End-to-End UART Servo Control:** Verified Compact Serial Protocol framing, `0xAA` auto-baud handshake, speed/acceleration profiling, and physical MG90S actuation across all 6 Pololu Maestro channels (`0..=5`).
- [x] **Direct ESP32 LEDC Servo Driver:** Implemented and bench-verified 50 Hz 14-bit `LedcServo` on `GPIO 25` conforming to `ServoActuator`.
- [x] **Split Power Rail Validation:** Verified external servo power injection (`VSRV`/`BAT`) with common ground isolation to prevent MCU brownouts.
- [ ] **Appendage Controller Integration:** Bind calibrated `WallEMap` channels into `WallEBody` and implement coordinated expressions (`look_at`, `tilt_eyes`, `wave`).
- [ ] **Tread Drive & Differential Kinematics:** Complete `tb67h450.rs` (`20 kHz` LEDC PWM) and differential tread mixer.
- [ ] **Full Chassis Assembly & Wiring Harness:** Final mechanical horn indexing at `0.5` neutral home and internal harness routing.
