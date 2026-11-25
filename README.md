# SW@GL0RD WALL-E — Rust Firmware (ESP32 + FreeRTOS)

Modular embedded Rust firmware (`esp-idf-hal` / `esp-idf-svc` on FreeRTOS) for a 3D-printed WALL-E robot powered by an **Emakefun Maker-ESP32**, a **Pololu Micro Maestro 6-channel servo controller**, an onboard **LEDC PWM servo**, and a **1.8" ST7735 SPI TFT diagnostics HUD**.

---

## Hardware Architecture & Pinout

### 1. Servo Actuators (7-DOF Upper Body)
* **Channels `0..=5` (Pololu Micro Maestro):** Driven over hardware `UART1` at **115,200 baud** using the Pololu Compact Serial Protocol (`init_auto_baud()`).
* **Channel `6` (`arm_right`):** Driven directly from the ESP32 `LEDC` peripheral (`Timer0`, `Channel0`) at **50 Hz** with **14-bit duty resolution**.

| Joint Name | Controller | Channel / Pin | Default Range (`µs`) |
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