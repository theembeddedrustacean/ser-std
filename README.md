# Simplified Embedded Rust: ESP Standard Library Edition - Book Repository 🦀 

<p align="center">
<img src="https://media.beehiiv.com/cdn-cgi/image/fit=scale-down,format=auto,onerror=redirect,quality=80/uploads/asset/file/7ea4745a-de35-4055-b260-32961ee3725b/1.png?t=1714653623" alt="BookCover" width="200"/>
</p>

Welcome to the _**Simplified Embedded Rust: ESP Standard Library Edition**_ book repository. Here you will find all the resources related to the book. You can get a copy of the book [here](https://www.theembeddedrustacean.com/c/ser-std).

## 📝 Reporting Issues & Content Suggestions
If you find any text errors, typos, or formatting issues in the book, please [report a text error here](https://github.com/theembeddedrustacean/ser-std/issues/new?assignees=&labels=documentation&projects=&template=text-error.md&title=) so that it can be addressed in a later revision. 

If you find any code issues in the book, please [report a bug here](https://github.com/theembeddedrustacean/ser-std/issues/new?assignees=&labels=bug&projects=&template=bug_report.md&title=) so that it can be addressed in a later revision. 

You are also welcome to [suggest a feature here](https://github.com/theembeddedrustacean/ser-std/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.md&title=) so it may be considered for content in the future.

## 🔗 GitHub Project Links
This is a list of the project links containing the example source code for the ESP32-C3 and the ESP32. This covers both RISC-V and Xtensa architechtures. Apart from that, variations among devices for the examples in the book are minor. All projects were setup using VS Code as an editor. Each branch contains the same collection of code examples accomodated for the different ESP device. Click on the link for the device you desire to work with and clone that particular branch.
| Device   | Devkit | GitHub Links |
| -------- | ------ | ------------ | 
| ESP32-C3 | [ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html) | [ESP32-C3 Branch](https://github.com/theembeddedrustacean/ser-std/tree/esp32c3dkm1) | 
| ESP32    | [ESP32-Dev-KitC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-devkitc.html#get-started-esp32-devkitc-board-front) | [ESP32 Branch](https://github.com/theembeddedrustacean/ser-std/tree/esp32dkc) | 

## 🔗 Wokwi Project Links
This is a list of links to the Wokwi examples presented in each chapter. These are based on the ESP32-C3 device and would need to be rewired for other devices. The Wokwi setup for the different devices can be also extracted from the VSCode projects. If you notice a template missing or would like to request one, feel free to submit a feature request.
| Chapter | Example Link |
| ------- | ------------ |
| **CH 5 - GPIO** | [Blinky](https://wokwi.com/projects/390146357737013249)<br>[LED Bar Blinking](https://wokwi.com/projects/390227640396906497)<br>[Button Press Counter](https://wokwi.com/projects/390146514698384385) |
| **CH 6 - ADC** | [Simple Voltmeter](https://wokwi.com/projects/390334806232301569)<br>[Temperature Sensing](https://wokwi.com/projects/390227886760909825) |
| **CH 7 - Timers & Counters** | [Pulse Duration Monitor](https://wokwi.com/projects/390192668237908993)<br>[Real-Time Timer](https://wokwi.com/projects/390145691083737089) |
| **CH 8 - PWM** | [LED Fading](https://wokwi.com/projects/388253411769572353) |
| **CH 9 - Serial Communication** | [UART XOR Cipher](https://wokwi.com/projects/390231605149450241)<br>[Interacting with an I2C RTC](https://wokwi.com/projects/390232372639107073) |
| **CH 10 - IoT and Networking** | [Connecting to WiFi](https://wokwi.com/projects/390233808897991681)<br>[Simple HTTP Client](https://wokwi.com/projects/390235149956598785)<br>[Simple HTTP Server](https://wokwi.com/projects/390235381687723009)<br>[Syncronizing System Time](https://wokwi.com/projects/390237075173881857)|

## 🔌 Wiring Templates for End of Chapter Questions
These are pre-wired templates to get you started with end of chapter questions. Questions that are not included would use exisiting book examples as templates. These are based on the ESP32-C3 device and would need to be rewired for other devices. If you notice a template missing or would like to request one, feel free to submit a feature request. 

| Chapter | Question Links |
| ------- | ---------------------- |
| **CH 5 - GPIO** | [Q5](https://wokwi.com/projects/397692235770967041)<br>[Q6](https://wokwi.com/projects/397692838510337025)<br>[Q8 & Q9](https://wokwi.com/projects/397699927017798657)| 
| **CH 6 - ADC** | [Q4](https://wokwi.com/projects/397702162614524929)<br>[Q5](https://wokwi.com/projects/397702270325303297)<br>[Q6 & Q7](https://wokwi.com/projects/397746259094848513) |
| **CH 7 - Timers & Counters** | [Q2](https://wokwi.com/projects/397703448954162177) |
| **CH 8 - PWM** | [Q3](https://wokwi.com/projects/397746565882019841)<br>[Q4](https://wokwi.com/projects/397746669510718465)<br>[Q5](https://wokwi.com/projects/397746791518866433)<br>[Q6](https://wokwi.com/projects/397746961987996673) |
| **CH 9 - Serial Communication** | 🤷‍♂️ |
| **CH 10 - IoT and Networking** | 🤷‍♂️ |

## 🧑‍💻 Development Options

> ⚠️ As of January 2025, Wowki stopped supporting the std builder on it's web interface. While the no_std builder is still supported on the web interface, an alternative beginner friendly option of using DevContainers and Wokwi with VSCode is introduced. With a few clicks in GitHub, Devcontainers enable the spawning of a full environment with all the book examples in a single workspace. Still, no need for any hardware and knowledge of Devcontainers is not required.

### 1. 🌐 Development Containers (Recommended)
This is the recommended option for beginners as it is the quickest and easiest way to get started. The full environment will be setup in your web browser with a few clicks and without leaving this repository. All you have to do is click on the green "Code" button in the upper right corder then navigate to the codespaces tab and click "create codespace on [device name]". A tab will automatically open setting up the whole environment with the example projects for you.

### 2. 🏡🛠️ Local Editor with Physical Hardware  
If you prefer to develop locally with physical hardware, you can clone the examples locally and set them up to run on an external development board. The software required entails the ESP-IDF framework in addition to flashing tools to download code to the external hardware. The following links include the instructions for installing the ESP-IDF framework and flashing the development board.

#### a) **Install Rust** 🦀: 
If you do not have Rust installed already, follow the instructions on the [rustup
website](https://www.rust-lang.org/tools/install).

#### b) **Install `espup`**: 
Run the following command in a terminal window:

```shell
cargo install espup
```

#### c) **Install Toolchains**: 
Run the following command in a terminal window:

```shell
espup install
```

#### d) **Install [Python](https://www.python.org/downloads/)**.

#### e) **Install [git](https://git-scm.com/downloads)**.

#### f) **Install the `ldproxy` Crate**: 
Run the following command in a terminal window:

```shell
cargo install ldproxy
```

#### g) Set Up the Environment Variables:
`espup` will create an export file that contains some environment variables required to build projects. If you’re on Windows, there’s no need to do anything special. However, for Unix-based systems, the file is `$HOME/export-esp.sh`. You can add the environment variables to your shell profile directly by adding the content of `$HOME/export-esp.sh` to your shell’s profile:

```shell
cat $HOME/export-esp.sh >> [path to profile]
```
For example, if you are using the `zshrc` profile your command may look something like this:

```shell
cat $HOME/export-esp.sh >> ~/.zshrc
```
Afterward, its recommended that you restart your terminal session for changes to take effect.

⚠️ If you are using macOS or Linux there may be additional prerequisites for setting up `std` development. Refer to Step 1 under the following [link](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html).

#### h) **Install `espflash`**: 
Run the following command in a terminal window:
```shell
cargo install espflash
```

Afterward, the easiest way to flash an ELF binary, is to add `espflash` as your Cargo runner. This way, when enterning `cargo run`, the code would automaitcally perform the flashing after file generation. This is done by adding the following line to your `.cargo/config.toml` file:

```shell
[target.'cfg(any(target_arch = "riscv32", target_arch = "xtensa"))']
runner = "espflash flash --monitor"
```
#### g) **Install `cargo-generate`**:
When creating your own projects from scratch, it is highly recommended that you use `cargo-generate`. Through `cargo-generate` you can create new project templates pre-configured for any ESP device. Click on the link below for instructions to install and use `cargo-generate`.
To install `cargo-generate` run the following command:

```shell
cargo install cargo-generate
```
Afterward, to generate a `std` template run the following command:

```shell
cargo generate esp-rs/esp-idf-template cargo
```

### 3. 🏡🔮 Local Editor with Wokwi
If you prefer to develop locally with Wowki (no hardware), you can clone the examples locally and install the following extensions for the simulator:
- [VSCode Wokwi Extension](https://docs.wokwi.com/vscode/getting-started)
- [JetBrains Wokwi Plugin](https://plugins.jetbrains.com/plugin/23826-wokwi-simulator)

⚠️ Local development with Wokwi still requires the installation of the ESP-IDF framework, however, flashing tools are not required since external hardware is not involved. Also through `cargo-generate` you can generate projects pre-configured for Wokwi skipping the second part after extension installation.

### ⛔️ Important Note: 
Options 2, 3, and 4 are not recommended for beginners due to the added complexity and, in some cases, required installations. For example, installing the ESP-IDF framework in particular is quite involved and not necessarily always a smooth experience. 

## 🧱 Hardware Component List (Optional)
This is a list of the components used in the different examples in the book. Acquiring these components is **OPTIONAL** and recommended only after you are comfortable with the material. You will only need these components if you are interested in doing physical hardware development at a later time (options 2 and 3 listed in the development options section earlier).

| Component                      | Documenation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Purchase Links                                                                                                                               |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Development Board (Choose one) | [ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html)<br>[ESP32-C3-DevKit-RUST-1](https://github.com/esp-rs/esp-rust-board/tree/v1.2) | ESP32-C3-DevKitM-1 ([AliExpress](https://www.aliexpress.com/item/3256803802784795.html?gps-id=pcStoreJustForYou&scm=1007.23125.137358.0&scm_id=1007.23125.137358.0&scm-url=1007.23125.137358.0&pvid=887074bd-9830-45ec-a9a0-e51a3b262eaf&_t=gps-id:pcStoreJustForYou,scm-url:1007.23125.137358.0,pvid:887074bd-9830-45ec-a9a0-e51a3b262eaf,tpp_buckets:668%232846%238108%231977&pdp_npi=4%40dis%21USD%218.00%218.00%21%21%218.00%218.00%21%402101c5a417149333693898144eafe8%2112000027657818087%21rec%21US%214083593659%21AB&spm=a2g0o.store_pc_home.smartJustForYou_2008854986518.1005003989099547))<br>ESP32-C3-DevKit-RUST-1 ([AliExpress](https://www.aliexpress.com/item/3256804232027536.html?spm=a2g0o.productlist.main.3.16a72kn92kn9EZ&algo_pvid=2e8dd822-5908-4691-82bb-2d41220563ec&algo_exp_id=2e8dd822-5908-4691-82bb-2d41220563ec-1&pdp_npi=4%40dis%21USD%2119.80%2119.80%21%21%2119.80%2119.80%21%402103200517149332747751850ee5b9%2112000029115522071%21sea%21US%214083593659%21AB&curPageLogUid=mZH5BE9nmh7P&utparam-url=scene%3Asearch%7Cquery_from%3A))|
| Female to Male Wires           | N/A                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | [AliExpress](https://s.click.aliexpress.com/e/_DcZBsT1)                                                                                                                                   |
| Prototyping Breadboard         | N/A                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | [AliExpress](https://s.click.aliexpress.com/e/_Dcw29Sj)                                                                                                                                   |
| LEDs                           | [Datasheet](https://components101.com/diodes/5mm-round-led)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DkzxbBz)                                                                                                                                   |
| LED Bar                        | [Datasheet](https://components101.com/displays/led-bar-graph)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_Dd9Kx4n)                                                                                                                                   |
| Push Button                    | [Datasheet](https://components101.com/switches/push-button)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_Dmrtcip)                                                                                                                                   |
| Potentiometer                  | [Datasheet](https://components101.com/resistors/potentiometer)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DdlX2Hz)                                                                                                                                   |
| NTC Temperature Sensor                      | [Datasheet](https://components101.com/resistors/ntc-thermistor-10k)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DDejccb)                                                                                                                                   |
| DS1307                         | [Datasheet](https://components101.com/ics/ds1307-i2c-real-time-clock-rtc)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DEWxS7v)                                                                                                                                   |
