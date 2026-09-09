# 🦀 ESP32 Embedded Rust — Core Library Edition (`no_std`)

Este repositório contém meus estudos, exercícios e simulações do livro **"Simplified Embedded Rust: ESP Core Library Edition"**.

---

## 🛠️ Tecnologias & Ferramentas Utilizadas

* **Linguagem:** Rust (`no_std`)
* **Target Hardware:** ESP32-C3
* **HAL:** `esp-hal` (v0.19+)
* **Simulador:** [Wokwi](https://wokwi.com)
* **Ferramentas de Desenvolvimento:** Cargo, `espflash`, `wokwi-server`

---

## 📌 Exercícios & Módulos

- [x] **GPIO & Interrupções:** Controle de saídas digitais, leitura de botões e tratamento de interrupções de hardware.
- [x] **Timers & Contadores:** Manipulação do `TIMG0`/`SYSTIMER`, medição de tempo com `Instant` e cálculo de deltas via `.elapsed()`.
- [x] **PWM (Pulse Width Modulation):** Controle de brilho de LEDs e geração de sinais para servomotores/ESCs.
- [x] **Comunicação Serial & I2C:** Leitura de sensores externos (ex: RTC / temperatura) e transmissão UART.

---

## 🔬 Projeto em Destaque: Pisca-Pisca com Hardware Timer (Wokwi)

Implementação de controle de tempo não-bloqueante utilizando os timers de hardware do ESP32-C3 e a API moderna do `esp-hal`.

### 📱 Esquema do Circuito no Wokwi

> **Conexões do Hardware:**
> * **GPIO1** ➔ Resistor 200Ω ➔ Anodo do LED Red
> * **GND** ➔ Catodo do LED Red

<!-- Adicione a imagem do seu circuito salvando-a na pasta 'assets' do repositório -->
<img width="2040" height="798" alt="image" src="https://github.com/user-attachments/assets/d79fde98-fc65-430d-984a-fbed1e78e717" />

---
